//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1105/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1105(t11597: f64, t14469: f64, t14470: f64, t1556: f64, t1734: f64, t1832: f64, t1859: f64, t18797: f64, t1881: f64, t2211: f64, t2630: f64, t2645: f64, t2675: f64, t2765: f64, t411: f64, t4441: f64, t5735: f64, t5783: f64, t6086: f64, t6087: f64, t6089: f64, t6129: f64, t7075: f64, t756: f64, t770: f64, t777: f64, t7886: f64, t7887: f64, t7977: f64, t8004: f64, t9163: f64) -> f64 {
    let t20586 = 9.0_f64 * t5735 * t8004 - 2.0_f64 * t2645 * t6087 - 3.0_f64 * t5783 * t2765 * t2630 * t411 - 2.7743564462147594_f64 * t11597 + t1881 * t7887 - 9.0_f64 * t5783 * t2765 * t770 * t1832 - 2.0_f64 * t777 * t18797 * t2675 - 2.0_f64 * t777 * t6086 * t6129 - 9.0_f64 * t5783 * t2765 * t1859 * t756 - 2.0_f64 * t2645 * t7075 - t9163 + 9.0_f64 * t6089 * t4441 - t7977 * t1556 + 3.0_f64 * t2211 * t7886 * t1734 + t14469 + 0.17961351015381913_f64 * t14470;
    t20586
}
