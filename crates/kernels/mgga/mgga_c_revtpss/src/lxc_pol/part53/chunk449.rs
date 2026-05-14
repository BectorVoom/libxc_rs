//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 449/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk449<F: Float>(t235: F, t2718: F, t231: F, t159: F, t243: F, t216: F, t2712: F, t785: F, t225: F, t826: F, t849: F, t820: F, t823: F, t843: F, t839: F, t241: F) -> (F, F, F, F, F, F, F, F, F) {
    let t2719 = t2718 * t235;
    let t2723 = t231 * t231;
    let t2729 = t159 * t243;
    let t2730 = t216 * t2729;
    let t2735 = t2712 * t785;
    let t2736 = t2735 * t225;
    let t2737 = t849 * t826;
    let t2739 = 0.25410001404642664112e-5 * t2736 * t2737;
    let t2741 = t820 * t823 * t843;
    let t2742 = t2741 * t839;
    let t2745 = t820 * t823 * t241;
    (t2719, t2723, t2730, t2735, t2736, t2739, t2741, t2742, t2745)
}
