//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1155/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1155<F: Float>(t100535: F, t106701: F, t107226: F, t107283: F, t107496: F, t1089: F, t12079: F, t12168: F, t1651: F, t1652: F, t1696: F, t19482: F, t23607: F, t23621: F, t23640: F, t25464: F, t25671: F, t25672: F, t27604: F, t27609: F, t27621: F, t29731: F, t29822: F, t29852: F, t29888: F, t3318: F, t6299: F, t6305: F, t6350: F, t7102: F, t7151: F, t7159: F, t7160: F, t7167: F, t7810: F, t7822: F, t93471: F, t93870: F, t93897: F) -> (F,) {
    let t113819 = -0.39512695097613069591e1 * t7102 * t23607 + 0.52041769129231196772e1 * t27609 * t29888 - 0.26020884564615598386e1 * t27621 * t29822 - 0.26020884564615598386e1 * t93471 * t93870 * t23640 * t12168 + 0.26020884564615598386e1 * t93471 * t25672 * t23640 * t12079 - 0.19756347548806534796e1 * t106701 * t1696 - 0.26020884564615598386e1 * t93897 * t107226 * t19482 * t1651 - 0.78062653693846795158e1 * t7159 * t25464 * t7810 * t6350 - 0.52041769129231196772e1 * t7151 * t7160 * t29731 * t1651 + 0.19756347548806534796e1 * t7102 * t23621 - 0.39512695097613069591e1 * t107283 * t1652 + 0.52041769129231196772e1 * t107496 * t7822 + 0.13010442282307799193e1 * t100535 * t29852 + 0.13010442282307799193e1 * t25671 * t27604 * t6305 * t3318 - 0.13010442282307799193e1 * t7167 * t27604 * t6299 * t1089;
    (t113819,)
}
