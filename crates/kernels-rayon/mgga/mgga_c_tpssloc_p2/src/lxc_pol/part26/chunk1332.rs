//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1332/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1332(t12823: f64, t6525: f64, t12734: f64, t1983: f64, t22578: f64, t6996: f64, t22480: f64, t2314: f64, t22947: f64, t532: f64, t6879: f64, t1874: f64, t39235: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t83919 = 6.0_f64 * t12823 * t6525;
    let t83921 = 12.0_f64 * t12734 * t6525;
    let t83924 = 3.0_f64 * t1983 * t6996 * t22578;
    let t83928 = 6.0_f64 * t2314 * t22480;
    let t83929 = t532 * t22947;
    let t83932 = 9.0_f64 * t1983 * t83929 * t6879;
    let t83939 = 2.0_f64 * t39235 * t1874;
    (t83919, t83921, t83924, t83928, t83932, t83939)
}
