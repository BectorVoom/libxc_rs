//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 824/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk824(t125: f64, t1550: f64, t169: f64, t1808: f64, t1881: f64, t2208: f64, t2211: f64, t2785: f64, t2791: f64, t2793: f64, t2806: f64, t281: f64, t299: f64, t301: f64, t411: f64, t4117: f64, t456: f64, t5464: f64, t5487: f64, t5490: f64, t5495: f64, t5499: f64, t5670: f64, t5673: f64, t5679: f64, t5682: f64, t5718: f64, t757: f64, t777: f64) -> f64 {
    let t5727 = t2785 + 2.0_f64 * t1881 * t1550 + t777 * t2791 + (t5464 + t5487) * t125 + 6.0_f64 * t5490 * t757 + 3.0_f64 * t4117 * t2208 + 12.0_f64 * t1808 * t5495 * t411 + 6.0_f64 * t1808 * t5499 + t5670 * t456 - 0.01197423401025461_f64 * t281 * t5673 - t5679 - 0.01197423401025461_f64 * t5682 + 0.020267214298646783_f64 * t169 * t299 * t5718 * t301 - 2.0_f64 * t777 * t2806 + 6.0_f64 * t2211 * t2793;
    t5727
}
