//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 824/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk824<F: Float>(t125: F, t1550: F, t169: F, t1808: F, t1881: F, t2208: F, t2211: F, t2785: F, t2791: F, t2793: F, t2806: F, t281: F, t299: F, t301: F, t411: F, t4117: F, t456: F, t5464: F, t5487: F, t5490: F, t5495: F, t5499: F, t5670: F, t5673: F, t5679: F, t5682: F, t5718: F, t757: F, t777: F) -> F {
    let t5727 = t2785 + F::new(2.0) * t1881 * t1550 + t777 * t2791 + (t5464 + t5487) * t125 + F::new(6.0) * t5490 * t757 + F::new(3.0) * t4117 * t2208 + F::new(12.0) * t1808 * t5495 * t411 + F::new(6.0) * t1808 * t5499 + t5670 * t456 - F::cast_from(0.01197423401025461_f64) * t281 * t5673 - t5679 - F::cast_from(0.01197423401025461_f64) * t5682 + F::cast_from(0.020267214298646783_f64) * t169 * t299 * t5718 * t301 - F::new(2.0) * t777 * t2806 + F::new(6.0) * t2211 * t2793;
    t5727
}
