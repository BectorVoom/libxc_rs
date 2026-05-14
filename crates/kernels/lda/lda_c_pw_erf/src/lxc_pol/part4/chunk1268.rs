//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1268/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1268<F: Float>(t142: F, t6121: F, t455: F, t1549: F, t6097: F, t169: F, t242: F, t299: F, t6039: F, t462: F, t6080: F, t18784: F, t6035: F, t632: F, t10881: F, t10883: F, t10886: F, t10897: F, t10903: F, t10906: F, t10909: F, t10913: F, t10915: F, t10918: F, t10922: F, t10956: F, t10961: F, t10963: F) -> (F, F, F, F) {
    let t18900 = t142 * t6121;
    let t18901 = t455 * t18900;
    let t18906 = t1549 * t6097;
    let t18918 = t169 * t299 * t6039 * t242;
    let t18920 = t462 * t6080;
    let t18923 = t169 * t18784 * t242;
    let t18934 = t169 * t6035 * t632;
    let t18936 = -2.55960325162461 * t10963 + 0.10611888591559791 * t18918 - 0.2133002709687175 * t18920 - 0.14149184788746388 * t18923 + t10897 + 0.10611888591559791 * t10903 + 0.3183566577467937 * t10906 + 1.0376068845080684 * t10909 + t10913 - 0.14149184788746388 * t10915 - 0.8489510873247833 * t10918 - t10922 - t10956 - t10881 - 0.031835665774679375 * t10883 - 0.06367133154935875 * t10886 + 0.31995040645307626 * t10961 + 0.10611888591559791 * t18934;
    (t18900, t18901, t18906, t18936)
}
