//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 584/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk584<F: Float>(t142: F, t770: F, t1554: F, t1101: F, t1108: F, t1146: F, t1148: F, t125: F, t145: F, t169: F, t1729: F, t2208: F, t2211: F, t2222: F, t2229: F, t2233: F, t2357: F, t2364: F, t2375: F, t242: F, t2589: F, t2592: F, t2595: F, t2645: F, t2647: F, t2673: F, t279: F, t281: F, t296: F, t299: F, t301: F, t405: F, t456: F, t777: F) -> (F, F, F) {
    let t2675 = t142 * t770;
    let t2676 = t1554 * t2675;
    let t2680 = 0.020267214298646783 * t169 * t299 * t2357 * t301 + (-t1101 + 0.10611888591559791 * t2222 + t1108 - 0.031835665774679375 * t169 * t2364 * t242 - 0.06367133154935875 * t2229 - t1146 + t1148 - 0.2133002709687175 * t2233 + 0.05332506774217938 * t145 * t2357) * t296 - 0.01197423401025461 * t281 * t2375 + t2589 * t279 + t777 * t2592 + 6.0 * t1729 * t2595 + t2645 * t456 + 3.0 * t405 * t2647 + t2673 * t125 - t777 * t2676 + 6.0 * t2211 * t2208;
    (t2675, t2676, t2680)
}
