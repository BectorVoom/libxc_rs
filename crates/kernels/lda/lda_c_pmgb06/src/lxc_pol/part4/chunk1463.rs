//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1463/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1463<F: Float>(t377: F, t7041: F, t2255: F, t1295: F, t2718: F, t2247: F, t5858: F, t7081: F, t11477: F, t11485: F, t11488: F, t11491: F, t1227: F, t18503: F, t18507: F, t18518: F, t18571: F, t18580: F, t18586: F, t18590: F, t2248: F, t2448: F, t8428: F, t8433: F, t8439: F) -> (F, F, F, F) {
    let t18796 = t7041 * t377;
    let t18804 = t2255 * t2255;
    let t18807 = t2718 * t1295;
    let t18815 = t2247 * t5858 * t7081;
    let t18823 = t18503 + t8428 - t18507 + t18518 - t18571 + t18580 + F::new(13.79404) * t11477 + F::cast_from(9.196026666666667_f64) * t11485 - F::new(6.89702) * t11488 - F::new(3.44851) * t11491 - F::new(3.44851) * t18815 + F::new(5.172765) * t2247 * t2248 * t2448 * t1227 + t18586 - t18590 + F::cast_from(2.2990066666666666_f64) * t8433 + F::cast_from(3.5762325925925924_f64) * t8439;
    (t18796, t18804, t18807, t18823)
}
