//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 979/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk979(t1081: f64, t5701: f64, t1772: f64, t3007: f64, t5677: f64, t684: f64, t5681: f64, t1738: f64, t2306: f64, t1729: f64, t1880: f64, t405: f64, t6153: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t14447 = t5701 * t1081;
    let t14448 = 0.0007324622014701264_f64 * t14447;
    let t14449 = t1772 * t3007;
    let t14468 = t684 * t5677;
    let t14469 = 0.11974234010254609_f64 * t14468;
    let t14470 = t684 * t5681;
    let t14472 = t1738 * t2306;
    let t14473 = 0.15965645347006147_f64 * t14472;
    let t14480 = t1729 * t1880;
    let t14485 = t405 * t6153;
    (t14448, t14449, t14469, t14470, t14473, t14480, t14485)
}
