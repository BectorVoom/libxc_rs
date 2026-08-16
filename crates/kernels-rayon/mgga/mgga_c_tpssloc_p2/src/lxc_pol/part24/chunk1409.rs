//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1409/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1409(t25: f64, t1965: f64, t2250: f64, t23773: f64, t40: f64, t607: f64, t6835: f64, t82334: f64, t83544: f64, t9258: f64, t1081: f64, t2752: f64, t13487: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64) {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t83554 = piecewise3(t115, t82334, t83544 * t40 / 2.0_f64 + 3.0_f64 / 2.0_f64 * t23773 * t607 + 3.0_f64 / 2.0_f64 * t6835 * t2250 + t1965 * t9258 / 2.0_f64);
    let t83555 = t2752 * t1081;
    let t83556 = t83555 * t13487;
    (t83554, t83556)
}
