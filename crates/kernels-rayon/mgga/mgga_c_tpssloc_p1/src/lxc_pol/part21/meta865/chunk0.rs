//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3157/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3157(t14831: f64, t4869: f64, t18915: f64, t3423: f64, t1164: f64, t14854: f64, t44154: f64, t6068: f64, t18280: f64, t3411: f64, t15041: f64, t11433: f64, t18279: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t65299 = 0.11696447245269292414e1_f64 * t4869 * t14831;
    let t65301 = 0.17315859105681463759e2_f64 * t18915 * t3423;
    let t65305 = 0.12304822629859687989e5_f64 * t1164 * t44154 * t6068 * t14854;
    let t65307 = 0.20779030926817756511e3_f64 * t3411 * t18280;
    let t65309 = 0.34631718211362927517e2_f64 * t4869 * t15041;
    let t65312 = 0.10389515463408878255e3_f64 * t1164 * t18279 * t11433;
    (t65299, t65301, t65305, t65307, t65309, t65312)
}
