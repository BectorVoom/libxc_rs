//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 756/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk756<F: Float>(t118: F, t6946: F, t2414: F, t409: F, t419: F, t421: F, t117: F, t123: F, t2687: F, t315: F, t2777: F, t2816: F, t3474: F, t3481: F, t5610: F, t5615: F, t5620: F, t5622: F, t5625: F, t5627: F, t5697: F, t5698: F, t5701: F, t5702: F) -> (F, F, F, F, F) {
    let t7153 = t6946 * t118;
    let t7155 = t409 * t2414;
    let t7157 = t7155 * t419 * t421;
    let t7167 = t123 * t315 * t2687 * t117;
    let t7170 = t5610 - F::new(0.02394846802050922) * t3474 + F::new(0.031505407223141116) * t7153 - F::new(0.001975389032890948) * t7157 + F::new(0.013169260219272987) * t5615 - t5620 - F::new(0.007901556131563792) * t5622 - F::new(0.0009908551388980995) * t5625 - F::new(0.12602162889256446) * t5627 - t5697 - F::new(0.06301081444628223) * t5698 + t5701 + F::new(0.12602162889256446) * t5702 + t2777 + F::new(0.008980675507690957) * t7167 + t3481 + F::new(0.06301081444628223) * t2816;
    (t7153, t7155, t7157, t7167, t7170)
}
