//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1456/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1456(t1227: f64, t2703: f64, t38: f64, t4394: f64, t776: f64, t342: f64, t6979: f64, t11376: f64, t11379: f64, t11390: f64, t11370: f64, t11373: f64, t11382: f64, t11388: f64, t11393: f64, t1282: f64, t5980: f64, t63: f64, t6996: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t18671 = 17.53815_f64 * t38 * t2703 * t1227;
    let t18674 = 11.6921_f64 * t38 * t776 * t4394;
    let t18677 = 11.6921_f64 * t38 * t6979 * t342;
    let t18684 = 0.9743416666666667_f64 * t11376;
    let t18685 = 1.2991222222222223_f64 * t11379;
    let t18688 = 3.031285185185185_f64 * t11390;
    let t18690 = 5.87616_f64 * t63 * t6996 * t1227 - t18671 + t18674 + t18677 + 11.75232_f64 * t63 * t1282 * t5980 * t342 + 1.46904_f64 * t11370 - 1.95872_f64 * t11373 + t18684 + t18685 + t11382 / 3.0_f64 + 4.570346666666667_f64 * t11388 + t18688 + 28.0_f64 / 27.0_f64 * t11393;
    (t18671, t18674, t18677, t18684, t18685, t18688, t18690)
}
