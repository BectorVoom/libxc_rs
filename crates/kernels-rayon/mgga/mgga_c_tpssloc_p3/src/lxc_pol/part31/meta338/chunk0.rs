//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1243/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1243(t13913: f64, t973: f64, t13552: f64, t13550: f64, t13644: f64, t1036: f64, t4622: f64, t3117: f64, t4571: f64, t248: f64, t3051: f64, t4347: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13915 = 0.55555555555555555554e-3_f64 * t973 * t13913;
    let t13921 = 2.0_f64 / 27.0_f64 * t13552;
    let t13922 = 4.0_f64 / 9.0_f64 * t13550;
    let t13923 = 2.0_f64 / 9.0_f64 * t13644;
    let t13946 = t4622 * t1036 / 432.0_f64;
    let t13948 = t3117 * t4571 / 3456.0_f64;
    let t13950 = t248 * t3051 * t4347;
    (t13915, t13921, t13922, t13923, t13946, t13948, t13950)
}
