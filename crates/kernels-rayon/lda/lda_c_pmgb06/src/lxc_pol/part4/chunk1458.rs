//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1458/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1458(t8355: f64, t8370: f64, t8374: f64, t11234: f64, t18649: f64, t1271: f64, t2712: f64, t955: f64, t350: f64, t365: f64, t7018: f64, t11230: f64, t8358: f64, t8376: f64, t8382: f64, t8386: f64, t8388: f64, t8390: f64) -> (f64, f64, f64, f64, f64) {
    let t18704 = 3.031285185185185_f64 * t8355;
    let t18706 = 1.2991222222222223_f64 * t8370;
    let t18707 = 0.6495611111111111_f64 * t8374;
    let t18716 = 70.1526_f64 * t11234 * t18649;
    let t18718 = t1271 * t2712 * t955;
    let t18721 = t365 * t7018 * t350;
    let t18723 = t18704 + 28.0_f64 / 27.0_f64 * t8358 + t18706 - t18707 + 3.91744_f64 * t8376 + 2.0_f64 / 3.0_f64 * t8382 + 1.95872_f64 * t8386 + 3.91744_f64 * t8388 - 0.97936_f64 * t8390 - 117.5232_f64 * t11230 * t18649 - t18716 + 1.95872_f64 * t18718 + 1.46904_f64 * t18721;
    (t18704, t18706, t18707, t18716, t18723)
}
