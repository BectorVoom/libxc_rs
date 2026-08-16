//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 716/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk716(t12716: f64, t3138: f64, t12499: f64, t151: f64, t12435: f64, t3086: f64, t3107: f64, t955: f64, t3216: f64, t196: f64, t967: f64, t119: f64, t181: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12717 = t3138 * t12716;
    let t12723 = t151 * t12499;
    let t12730 = t3086 * t12435;
    let t12734 = t955 * t3107;
    let t12735 = t3216 * t12734;
    let t12741 = t196 * t967;
    let t12747 = t119 * t181;
    (t12717, t12723, t12730, t12734, t12735, t12741, t12747)
}
