//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3157/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3157<F: Float>(t14831: F, t4869: F, t18915: F, t3423: F, t1164: F, t14854: F, t44154: F, t6068: F, t18280: F, t3411: F, t15041: F, t11433: F, t18279: F) -> (F, F, F, F, F, F) {
    let t65299 = F::cast_from(0.11696447245269292414e1_f64) * t4869 * t14831;
    let t65301 = F::cast_from(0.17315859105681463759e2_f64) * t18915 * t3423;
    let t65305 = F::cast_from(0.12304822629859687989e5_f64) * t1164 * t44154 * t6068 * t14854;
    let t65307 = F::cast_from(0.20779030926817756511e3_f64) * t3411 * t18280;
    let t65309 = F::cast_from(0.34631718211362927517e2_f64) * t4869 * t15041;
    let t65312 = F::cast_from(0.10389515463408878255e3_f64) * t1164 * t18279 * t11433;
    (t65299, t65301, t65305, t65307, t65309, t65312)
}
