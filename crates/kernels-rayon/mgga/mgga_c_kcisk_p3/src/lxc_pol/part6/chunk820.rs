//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 820/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk820(t3679: f64, t7785: f64, t1354: f64, t7710: f64, t443: f64, t8102: f64, t7706: f64, t3973: f64, t8044: f64, t1309: f64, t6157: f64, t6171: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t25894 = t7785 * t3679;
    let t25921 = t1354 * t7710;
    let t25925 = t443 * t8102;
    let t25947 = t1354 * t7706;
    let t25980 = t3973 * t8044;
    let t25981 = t1309 * t25980;
    let t25985 = t6157 * t6171;
    (t25894, t25921, t25925, t25947, t25981, t25985)
}
