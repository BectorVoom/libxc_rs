//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1106/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1106(t1553: f64, t2642: f64, t169: f64, t301: f64, t717: f64, t7387: f64, t684: f64, t7339: f64, t142: f64, t14473: f64, t14485: f64, t14488: f64, t1550: f64, t1554: f64, t1555: f64, t18805: f64, t1881: f64, t18901: f64, t18906: f64, t19449: f64, t2211: f64, t2592: f64, t2765: f64, t2805: f64, t411: f64, t440: f64, t5735: f64, t5783: f64, t6025: f64, t6098: f64, t7166: f64, t7214: f64, t777: f64, t7880: f64, t7977: f64, t7986: f64, t7988: f64, t7991: f64) -> f64 {
    let t20589 = t1553 * t2642;
    let t20608 = t169 * t717 * t7387 * t301;
    let t20618 = t684 * t7339;
    let t20628 = -t14473 + 9.0_f64 * t5735 * t6098 - t777 * t20589 * t1555 + 9.0_f64 * t2211 * t18906 - t777 * t2805 * t7991 + 9.0_f64 * t2211 * t18901 - t777 * t1554 * t142 * t7166 + 2.0_f64 * t1881 * t7988 + 2.0_f64 * t7214 * t2592 + t7977 * t1550 - 0.054045904796391424_f64 * t20608 - 9.0_f64 * t5783 * t18805 + 6.0_f64 * t14485 * t2765 * t7986 * t411 + 18.0_f64 * t6025 * t19449 + 0.019957056683757683_f64 * t20618 - 18.0_f64 * t14488 * t2765 * t7880 * t411 + 18.0_f64 * t14485 * t2765 * t7880 * t440;
    t20628
}
