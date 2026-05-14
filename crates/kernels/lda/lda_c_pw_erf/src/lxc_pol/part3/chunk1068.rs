//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1068/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1068<F: Float>(t14472: F, t10838: F, t11619: F, t11648: F, t11661: F, t125: F, t14410: F, t14454: F, t14465: F, t14469: F, t14470: F, t1556: F, t169: F, t1881: F, t2779: F, t2801: F, t299: F, t301: F, t4449: F, t5670: F, t5735: F, t777: F, t9141: F, t9146: F, t9157: F, t9163: F, t9172: F) -> (F,) {
    let t14473 = 0.15965645347006147 * t14472;
    let t14475 = 6.0 * t1881 * t2779 + 6.0 * t777 * t9157 + 18.0 * t4449 * t9141 + (t11619 + t11648 + t11661 + t14410) * t125 + 0.020267214298646783 * t169 * t299 * t14454 * t301 - 0.054045904796391424 * t9146 - 3.0 * t5670 * t1556 - t9163 + 9.0 * t5735 * t2801 - 18.0 * t9172 * t14465 + t14469 + 0.05987117005127304 * t14470 - t14473 + 0.5945049527603057 * t10838;
    (t14475,)
}
