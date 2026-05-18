//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1197/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1197<F: Float>(t26: F, t7937: F, t7306: F, t76: F, t113: F, t301: F, t395: F, t7364: F, t11200: F, t1233: F, t14797: F, t15096: F, t15136: F, t18939: F, t21358: F, t2180: F, t2209: F, t2249: F, t2308: F, t2718: F, t2730: F, t2741: F, t329: F, t342: F, t346: F, t374: F, t384: F, t387: F, t389: F, t4042: F, t4354: F, t4360: F, t5583: F, t5992: F, t6006: F, t6007: F, t6008: F, t6018: F, t7354: F, t769: F, t77: F, t787: F, t7917: F, t8211: F) -> F {
    let t21655 = t7937 * t26;
    let t21676 = t76 * t7306;
    let t21682 = t395 * t7364 * t113 * t301;
    let t21699 = t8211 + F::new(4.0) * t6006 * t4042 * t787 * t6008 + F::new(6.0) * t21655 * t389 + F::new(18.0) * t2180 * t14797 * t769 + F::new(18.0) * t2180 * t5992 * t2209 + F::new(18.0) * t11200 * t7917 + F::new(2.0) * t6006 * t6007 * t2718 * t374 + F::new(18.0) * t1233 * t18939 * t4360 - t346 * t2308 * t2730 * t374 + F::new(6.0) * t2180 * t21676 * t342 - F::new(0.0002905674151788692) * t21682 + t346 * t7354 * t384 + F::new(3.0) * t329 * t77 * t21358 - F::new(2.0) * t346 * t15096 * t2741 - F::new(9.0) * t5583 * t15136 * t4354 + F::new(18.0) * t6018 * t387 * t787 * t2249;
    t21699
}
