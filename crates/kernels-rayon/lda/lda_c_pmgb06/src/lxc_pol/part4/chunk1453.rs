//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1453/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1453(t1227: f64, t1234: f64, t18609: f64, t18616: f64, t18622: f64, t18625: f64, t18628: f64, t18630: f64, t2712: f64, t2715: f64, t342: f64, t35: f64, t360: f64, t4394: f64, t6989: f64, t6996: f64, t7018: f64, t780: f64, t8263: f64) -> f64 {
    let t18632 = -6.0_f64 * t360 * t35 * t6996 * t1234 + 30.0_f64 * t360 * t35 * t6989 * t1234 - 6.0_f64 * t360 * t35 * t2712 * t1227 + 3.0_f64 * t360 * t35 * t780 * t4394 - t18609 + 3.0_f64 / 2.0_f64 * t360 * t35 * t2715 * t1227 - t18616 + 3.0_f64 * t360 * t35 * t7018 * t342 + 4.0_f64 * t18622 - 2.0_f64 * t18625 + 29.3808_f64 * t18628 - 11.75232_f64 * t18630 + t8263;
    t18632
}
