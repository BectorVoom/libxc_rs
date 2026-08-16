//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 768/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk768(t169: f64, t2208: f64, t2211: f64, t2595: f64, t2785: f64, t279: f64, t299: f64, t301: f64, t4441: f64, t4454: f64, t4455: f64, t4457: f64, t5679: f64, t5682: f64, t5735: f64, t5740: f64, t6080: f64, t6126: f64, t6130: f64, t6136: f64, t6140: f64, t6154: f64, t6156: f64, t7049: f64, t7057: f64, t777: f64) -> f64 {
    let t7060 = 6.0_f64 * t6126 * t2595 - t777 * t6130 + t4454 + 0.07982822673503073_f64 * t4455 - 0.10643763564670763_f64 * t4457 + t2785 - 0.054045904796391424_f64 * t6136 - 0.0002905674151788692_f64 * t6140 + 0.020267214298646783_f64 * t169 * t299 * t6080 * t301 + 6.0_f64 * t5735 * t2208 + 6.0_f64 * t2211 * t4441 + 6.0_f64 * t2211 * t5740 - t5679 - 0.02394846802050922_f64 * t5682 + 2.0_f64 * t6154 * t6156 + (t7049 + t7057) * t279;
    t7060
}
