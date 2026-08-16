//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1230/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1230(t14468: f64, t5681: f64, t684: f64, t1738: f64, t2306: f64, t10838: f64, t11619: f64, t11648: f64, t11661: f64, t125: f64, t14410: f64, t14454: f64, t14465: f64, t1556: f64, t169: f64, t1881: f64, t2779: f64, t2801: f64, t299: f64, t301: f64, t4449: f64, t5670: f64, t5735: f64, t777: f64, t9141: f64, t9146: f64, t9157: f64, t9163: f64, t9172: f64) -> f64 {
    let t14469 = 0.11974234010254609_f64 * t14468;
    let t14470 = t684 * t5681;
    let t14472 = t1738 * t2306;
    let t14473 = 0.15965645347006147_f64 * t14472;
    let t14475 = 6.0_f64 * t1881 * t2779 + 6.0_f64 * t777 * t9157 + 18.0_f64 * t4449 * t9141 + (t11619 + t11648 + t11661 + t14410) * t125 + 0.020267214298646783_f64 * t169 * t299 * t14454 * t301 - 0.054045904796391424_f64 * t9146 - 3.0_f64 * t5670 * t1556 - t9163 + 9.0_f64 * t5735 * t2801 - 18.0_f64 * t9172 * t14465 + t14469 + 0.05987117005127304_f64 * t14470 - t14473 + 0.5945049527603057_f64 * t10838;
    t14475
}
