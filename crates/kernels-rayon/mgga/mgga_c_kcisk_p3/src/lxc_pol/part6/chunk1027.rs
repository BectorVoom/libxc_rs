//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 1027/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk1027(t30801: f64, t30836: f64, t30855: f64, t30873: f64, t30153: f64, t425: f64, t2191: f64, t25947: f64, t2181: f64, t7710: f64, t25921: f64, t5646: f64, t7897: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t30875 = t30801 + t30836 + t30855 + t30873;
    let t30877 = t425 * t30153;
    let t30880 = t25947 * t2191;
    let t30883 = t2181 * t7710;
    let t30886 = t25921 * t2191;
    let t30889 = t5646 * t7897;
    (t30875, t30877, t30880, t30883, t30886, t30889)
}
