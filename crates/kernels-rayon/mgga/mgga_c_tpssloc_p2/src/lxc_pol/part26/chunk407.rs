//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 407/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk407(t1893: f64, t1895: f64, t235: f64, t59: f64, t226: f64, t249: f64, t1888: f64) -> (f64, f64, f64) {
    let t1896 = t1893 * t1895;
    let t1898 = t235 * t59;
    let t1899 = t226 * t1898;
    let t1900 = t1899 * t249;
    let t1902 = t1888 / 96.0_f64 + 0.20186378047070195427e-3_f64 * t1896 + t1900 / 1536.0_f64;
    (t1898, t1899, t1902)
}
