//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 886/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk886<F: Float>(t3373: F, t4092: F, t4095: F, t4096: F, t4099: F, t4103: F, t4106: F, t4113: F, t5894: F, t5897: F, t5898: F, t5904: F, t5907: F, t5911: F, t169: F, t2208: F, t2211: F, t2595: F, t2785: F, t279: F, t299: F, t301: F, t4441: F, t4454: F, t4455: F, t4457: F, t5679: F, t5682: F, t5735: F, t5740: F, t6080: F, t6126: F, t6130: F, t6136: F, t6140: F, t6154: F, t6156: F, t7049: F, t777: F) -> (F,) {
    let t7057 = -0.3350512821420176 * t5894 + t5897 + 0.3350512821420176 * t5898 - t3373 + 2.657442045789236 * t5904 - 0.10611888591559791 * t5907 - t5911 - 0.0837628205355044 * t4092 - t4095 - 0.1675256410710088 * t4096 - t4099 + 0.1675256410710088 * t4103 + t4106 + t4113;
    let t7060 = 6.0 * t6126 * t2595 - t777 * t6130 + t4454 + 0.07982822673503073 * t4455 - 0.10643763564670763 * t4457 + t2785 - 0.054045904796391424 * t6136 - 0.0002905674151788692 * t6140 + 0.020267214298646783 * t169 * t299 * t6080 * t301 + 6.0 * t5735 * t2208 + 6.0 * t2211 * t4441 + 6.0 * t2211 * t5740 - t5679 - 0.02394846802050922 * t5682 + 2.0 * t6154 * t6156 + (t7049 + t7057) * t279;
    (t7060,)
}
