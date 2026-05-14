//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1022/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1022<F: Float>(t1322: F, t787: F, t117: F, t123: F, t2687: F, t740: F, t1179: F, t2414: F, t419: F, t421: F, t409: F, t6716: F, t1186: F, t7155: F, t10670: F, t10790: F, t14275: F, t14279: F, t14283: F, t14287: F, t14290: F, t14293: F, t14297: F, t14300: F, t14303: F, t14306: F, t14308: F) -> (F, F) {
    let t15136 = t1322 * t787;
    let t15152 = t123 * t740 * t2687 * t117;
    let t15159 = t1179 * t2414 * t419 * t421;
    let t15163 = t409 * t6716 * t419 * t421;
    let t15166 = t7155 * t1186 * t421;
    let t15168 = 0.1756220988170676 * t10670 - 0.051799090195807085 * t14275 + 0.006935985972286697 * t14279 - 0.001981710277796199 * t14283 - 0.003950778065781896 * t14287 - 0.015803112263127583 * t14290 - 0.01185233419734569 * t14293 + 0.026338520438545975 * t14297 + 0.03950778065781896 * t14300 - 0.002972565416694299 * t14303 - 0.02394846802050922 * t15152 - 0.02394846802050922 * t10790 + 0.3780648866776934 * t14306 - 0.0002373061974330281 * t14308 + 0.006584630109636494 * t15159 - 0.003950778065781896 * t15163 - 0.003950778065781896 * t15166;
    (t15136, t15168)
}
