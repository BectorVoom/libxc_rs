//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 768/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk768<F: Float>(t169: F, t2208: F, t2211: F, t2595: F, t2785: F, t279: F, t299: F, t301: F, t4441: F, t4454: F, t4455: F, t4457: F, t5679: F, t5682: F, t5735: F, t5740: F, t6080: F, t6126: F, t6130: F, t6136: F, t6140: F, t6154: F, t6156: F, t7049: F, t7057: F, t777: F) -> F {
    let t7060 = F::new(6.0) * t6126 * t2595 - t777 * t6130 + t4454 + F::cast_from(0.07982822673503073_f64) * t4455 - F::cast_from(0.10643763564670763_f64) * t4457 + t2785 - F::cast_from(0.054045904796391424_f64) * t6136 - F::cast_from(0.0002905674151788692_f64) * t6140 + F::cast_from(0.020267214298646783_f64) * t169 * t299 * t6080 * t301 + F::new(6.0) * t5735 * t2208 + F::new(6.0) * t2211 * t4441 + F::new(6.0) * t2211 * t5740 - t5679 - F::cast_from(0.02394846802050922_f64) * t5682 + F::new(2.0) * t6154 * t6156 + (t7049 + t7057) * t279;
    t7060
}
