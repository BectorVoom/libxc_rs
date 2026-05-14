//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 700/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk700<F: Float>(t387: F, t7088: F, t384: F, t783: F, t2448: F, t73: F, t123: F, t1316: F, t2258: F, t2308: F, t2311: F, t295: F, t297: F, t315: F, t317: F, t346: F, t388: F, t4027: F, t4030: F, t5590: F, t5591: F, t5593: F, t5601: F, t6104: F, t6927: F, t6951: F, t6958: F, t6961: F, t787: F) -> (F, F, F, F) {
    let t7089 = t7088 * t387;
    let t7099 = t384 * t783;
    let t7102 = t73 * t2448;
    let t7106 = (t6927 + t6951) * t295 + t5590 + 0.07982822673503073 * t5591 - 0.10643763564670763 * t5593 - 0.01197423401025461 * t6958 - 0.01197423401025461 * t297 * t6961 - t4027 + t346 * t2258 * t787 + 0.19816831758676853 * t4030 + t346 * t7089 * t73 - t5601 + 0.020267214298646783 * t123 * t315 * t6104 * t317 + 6.0 * t1316 * t2258 * t2311 - t346 * t2308 * t7099 + 3.0 * t1316 * t388 * t7102;
    (t7089, t7099, t7102, t7106)
}
