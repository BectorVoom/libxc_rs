//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 746/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk746(t1282: f64, t2448: f64, t2703: f64, t342: f64, t38: f64, t2209: f64, t776: f64, t2707: f64, t5788: f64, t2229: f64, t360: f64, t5774: f64, t5785: f64, t5787: f64, t5791: f64, t5795: f64, t5797: f64, t63: f64, t6968: f64, t6971: f64, t6975: f64, t6978: f64, t6980: f64, t6984: f64, t6987: f64, t6989: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6996 = t1282 * t2448;
    let t7002 = 17.53815_f64 * t38 * t2703 * t342;
    let t7005 = 11.6921_f64 * t38 * t776 * t2209;
    let t7008 = 5.84605_f64 * t38 * t2707 * t342;
    let t7009 = 1.2991222222222223_f64 * t5788;
    let t7012 = -2.93808_f64 * t6968 + 0.73452_f64 * t6971 - t6975 + t6978 - t360 * t6980 / 2.0_f64 - t6984 / 2.0_f64 + t6987 / 6.0_f64 - 29.3808_f64 * t63 * t6989 * t342 + 11.75232_f64 * t63 * t2229 * t2209 + 5.87616_f64 * t63 * t6996 * t342 - t7002 + t7005 + t7008 - t5774 - t5785 - t5787 - t7009 - 4.0_f64 / 9.0_f64 * t5791 + t5795 - 0.97936_f64 * t5797;
    (t6996, t7002, t7005, t7008, t7009, t7012)
}
