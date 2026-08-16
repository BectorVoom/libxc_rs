//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 838/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk838(t28: f64, t265: f64, t504: f64, t1256: f64, t193: f64, t336: f64, t3640: f64, t8424: f64, t8900: f64, t8904: f64, t52: f64, t8434: f64, t8681: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> (f64, f64) {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t8909 = piecewise3(t505, t1256 * t193 * t336 * t8900 - t193 * t336 * t3640 * t8904, t8424);
    let t8912 = piecewise3(t401, t8434, t8909 * t52 / 2.0_f64);
    let t8913 = t8681 + t8912;
    (t8909, t8913)
}
