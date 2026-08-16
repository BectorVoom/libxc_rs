//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 565/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk565(t170: f64, t3128: f64, t159: f64, t1650: f64, t1662: f64, t1667: f64, t1671: f64, t1688: f64, t1695: f64, t1702: f64, t216: f64, t2738: f64, t2741: f64, t2744: f64, t2750: f64, t3034: f64, t3124: f64, t41: f64) -> (f64, f64) {
    let t3129 = t3128 * t170;
    let t3136 = -t41 * t3124 + t1650 - 0.21973736767207854065e-2_f64 * t3034 * t216 + 0.285764e-1_f64 * t159 * t3129 + 0.34631718211362927518e2_f64 * t2738 - t1662 + t1667 - t1671 + t1688 - 0.23392894490538584828e1_f64 * t2741 + 0.2701041328e0_f64 * t2744 - t1695 + 0.11696447245269292414e1_f64 * t2750 - t1702;
    (t3129, t3136)
}
