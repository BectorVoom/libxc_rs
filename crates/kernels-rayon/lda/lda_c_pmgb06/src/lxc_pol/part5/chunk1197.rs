//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1197/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1197(t26: f64, t7937: f64, t7306: f64, t76: f64, t113: f64, t301: f64, t395: f64, t7364: f64, t11200: f64, t1233: f64, t14797: f64, t15096: f64, t15136: f64, t18939: f64, t21358: f64, t2180: f64, t2209: f64, t2249: f64, t2308: f64, t2718: f64, t2730: f64, t2741: f64, t329: f64, t342: f64, t346: f64, t374: f64, t384: f64, t387: f64, t389: f64, t4042: f64, t4354: f64, t4360: f64, t5583: f64, t5992: f64, t6006: f64, t6007: f64, t6008: f64, t6018: f64, t7354: f64, t769: f64, t77: f64, t787: f64, t7917: f64, t8211: f64) -> f64 {
    let t21655 = t7937 * t26;
    let t21676 = t76 * t7306;
    let t21682 = t395 * t7364 * t113 * t301;
    let t21699 = t8211 + 4.0_f64 * t6006 * t4042 * t787 * t6008 + 6.0_f64 * t21655 * t389 + 18.0_f64 * t2180 * t14797 * t769 + 18.0_f64 * t2180 * t5992 * t2209 + 18.0_f64 * t11200 * t7917 + 2.0_f64 * t6006 * t6007 * t2718 * t374 + 18.0_f64 * t1233 * t18939 * t4360 - t346 * t2308 * t2730 * t374 + 6.0_f64 * t2180 * t21676 * t342 - 0.0002905674151788692_f64 * t21682 + t346 * t7354 * t384 + 3.0_f64 * t329 * t77 * t21358 - 2.0_f64 * t346 * t15096 * t2741 - 9.0_f64 * t5583 * t15136 * t4354 + 18.0_f64 * t6018 * t387 * t787 * t2249;
    t21699
}
