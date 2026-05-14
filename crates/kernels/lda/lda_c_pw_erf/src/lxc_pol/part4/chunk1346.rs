//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1346/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1346<F: Float>(t11217: F, t11219: F, t11222: F, t11225: F, t11227: F, t11229: F, t11232: F, t11233: F, t11236: F, t14965: F, t19388: F, t19397: F, t2594: F, t2765: F, t440: F, t100: F, t10838: F, t10840: F, t143: F, t14468: F, t14470: F, t14472: F, t14491: F, t1568: F, t1734: F, t1808: F, t18900: F, t19097: F, t19355: F, t19372: F, t19387: F, t2208: F, t2211: F, t2595: F, t2647: F, t279: F, t2793: F, t405: F, t411: F, t452: F, t5490: F, t5735: F, t6019: F, t6089: F, t6094: F, t6126: F, t7082: F, t7085: F, t776: F, t9163: F, t9172: F) -> (F,) {
    let t19399 = -0.0837628205355044 * t19388 + 1.0051538464260528 * t14965 - 0.1675256410710088 * t11217 - 1.0051538464260528 * t11219 - t11222 + 0.1675256410710088 * t11225 - 0.0837628205355044 * t11227 - 0.1675256410710088 * t11229 - t11232 + 0.5025769232130264 * t11233 + t11236 - 0.1675256410710088 * t19397;
    let t19421 = t2765 * t2594 * t440;
    let t19424 = 6.0 * t6089 * t2793 + 12.0 * t6126 * t452 * t6094 + 6.0 * t10840 * t6094 - t9163 + 6.0 * t5490 * t2647 + 3.0 * t405 * t143 * t19097 + 6.0 * t1568 * t100 * t2595 + (t19355 + t19372 + t19387 + t19399) * t279 + 0.15965645347006147 * t14468 + 0.11974234010254609 * t14470 - 0.21287527129341527 * t14472 + 12.0 * t5490 * t776 * t2208 + 6.0 * t2211 * t7082 * t1734 + 12.0 * t1808 * t18900 * t411 + 24.0 * t14491 * t7085 + 0.19816831758676853 * t10838 + 6.0 * t5735 * t6019 - 6.0 * t9172 * t19421;
    (t19424,)
}
