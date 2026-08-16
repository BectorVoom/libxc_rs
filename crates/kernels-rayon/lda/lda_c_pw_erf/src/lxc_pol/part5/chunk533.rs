//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 533/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk533(t142: f64, t770: f64, t1554: f64, t1101: f64, t1108: f64, t1146: f64, t1148: f64, t125: f64, t145: f64, t169: f64, t1729: f64, t2208: f64, t2211: f64, t2222: f64, t2229: f64, t2233: f64, t2357: f64, t2364: f64, t2375: f64, t242: f64, t2589: f64, t2592: f64, t2595: f64, t2645: f64, t2647: f64, t2673: f64, t279: f64, t281: f64, t296: f64, t299: f64, t301: f64, t405: f64, t456: f64, t777: f64) -> (f64, f64, f64) {
    let t2675 = t142 * t770;
    let t2676 = t1554 * t2675;
    let t2680 = 0.020267214298646783_f64 * t169 * t299 * t2357 * t301 + (-t1101 + 0.10611888591559791_f64 * t2222 + t1108 - 0.031835665774679375_f64 * t169 * t2364 * t242 - 0.06367133154935875_f64 * t2229 - t1146 + t1148 - 0.2133002709687175_f64 * t2233 + 0.05332506774217938_f64 * t145 * t2357) * t296 - 0.01197423401025461_f64 * t281 * t2375 + t2589 * t279 + t777 * t2592 + 6.0_f64 * t1729 * t2595 + t2645 * t456 + 3.0_f64 * t405 * t2647 + t2673 * t125 - t777 * t2676 + 6.0_f64 * t2211 * t2208;
    (t2675, t2676, t2680)
}
