//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1523/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1523(t13804: f64, t13845: f64, t13894: f64, t13937: f64, t225: f64, t68: f64, t369: f64, t1036: f64, t4622: f64, t3117: f64, t4571: f64, t248: f64, t3051: f64, t4347: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13939 = t13804 + t13845 + t13894 + t13937;
    let t13940 = t13939 * t225;
    let t13941 = t13940 * t68;
    let t13942 = t13941 * t369;
    let t13946 = t4622 * t1036 / 432.0_f64;
    let t13948 = t3117 * t4571 / 3456.0_f64;
    let t13950 = t248 * t3051 * t4347;
    (t13939, t13940, t13942, t13946, t13948, t13950)
}
