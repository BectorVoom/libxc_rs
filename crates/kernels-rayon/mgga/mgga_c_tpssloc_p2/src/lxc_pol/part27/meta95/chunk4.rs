//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 622/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk622(t25: f64, t265: f64, t504: f64, t1918: f64, t1965: f64, t40: f64, t1915: f64, t28: f64, t1877: f64, t1964: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t505 = t265 < t504;
    let t1968 = piecewise3(t115, t1918, t1965 * t40 / 2.0_f64);
    let t1969 = t1915 * t28;
    let t1971 = t1877 * t1969 / 2.0_f64;
    let t1972 = piecewise3(t505, 0.0_f64, t1964);
    (t1968, t1971, t1972)
}
