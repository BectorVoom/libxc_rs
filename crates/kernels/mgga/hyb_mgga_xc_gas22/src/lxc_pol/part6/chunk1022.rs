//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1022/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1022<F: Float>(t3: F, t3141: F, t163: F, t4014: F, t3997: F, t732: F, t166: F, t736: F, t169: F, t6270: F, t2098: F, t712: F, t151: F, t716: F, t154: F, t3165: F, t3188: F, t3191: F, t3211: F, t3216: F, t3221: F, t708: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t10212 = t3141 * t3;
    let t10218 = t163 * t4014;
    let t10221 = t732 * t3997;
    let t10226 = t166 * t4014;
    let t10229 = t736 * t3997;
    let t10234 = t169 * t4014;
    let t10237 = t6270 * t3997;
    let t10242 = t2098 * t4014;
    let t10245 = t712 * t3997;
    let t10250 = t151 * t4014;
    let t10253 = t716 * t3997;
    let t10258 = t154 * t4014;
    let t10261 = t10218 * t708 / 258048.0 + t10221 * t708 / 491520.0 - t3211 * t3188 / 3440640.0 - t10226 * t708 / 6881280.0 - t10229 * t708 / 13271040.0 + t3216 * t3188 / 0.10616832e9 + t10234 * t708 / 0.21233664e9 + t10237 * t708 / 412876800.0 - t3221 * t3188 / 0.37158912e10 - t10242 * t708 / 0.74317824e10 - 2.0 / 3.0 * t10245 * t708 + t3165 * t3188 / 3.0 + t10250 * t708 / 6.0 + t10253 * t708 / 8.0 - t3191 * t3188 / 24.0 - t10258 * t708 / 48.0;
    (t10212, t10218, t10221, t10226, t10229, t10234, t10237, t10242, t10245, t10250, t10253, t10258, t10261)
}
