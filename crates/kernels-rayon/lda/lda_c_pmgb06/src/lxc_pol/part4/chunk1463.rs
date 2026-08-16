//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1463/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1463(t377: f64, t7041: f64, t2255: f64, t1295: f64, t2718: f64, t2247: f64, t5858: f64, t7081: f64, t11477: f64, t11485: f64, t11488: f64, t11491: f64, t1227: f64, t18503: f64, t18507: f64, t18518: f64, t18571: f64, t18580: f64, t18586: f64, t18590: f64, t2248: f64, t2448: f64, t8428: f64, t8433: f64, t8439: f64) -> (f64, f64, f64, f64) {
    let t18796 = t7041 * t377;
    let t18804 = t2255 * t2255;
    let t18807 = t2718 * t1295;
    let t18815 = t2247 * t5858 * t7081;
    let t18823 = t18503 + t8428 - t18507 + t18518 - t18571 + t18580 + 13.79404_f64 * t11477 + 9.196026666666667_f64 * t11485 - 6.89702_f64 * t11488 - 3.44851_f64 * t11491 - 3.44851_f64 * t18815 + 5.172765_f64 * t2247 * t2248 * t2448 * t1227 + t18586 - t18590 + 2.2990066666666666_f64 * t8433 + 3.5762325925925924_f64 * t8439;
    (t18796, t18804, t18807, t18823)
}
