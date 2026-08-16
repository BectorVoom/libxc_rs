//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 709/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk709(t300: f64, t6091: f64, t6064: f64, t1703: f64, t4869: f64, t1156: f64, t3375: f64, t6068: f64, t1164: f64, t1147: f64, t6084: f64, t3400: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6092 = t300 * t6091;
    let t6094 = 0.19751673498613801407e-1_f64 * t300 * t6064;
    let t6096 = 0.11696447245269292414e1_f64 * t4869 * t1703;
    let t6098 = t3375 * t6068 * t1156;
    let t6100 = 0.11696447245269292414e1_f64 * t1164 * t6098;
    let t6102 = t1147 * t6084 * t1156;
    let t6104 = 0.5848223622634646207e0_f64 * t1164 * t6102;
    let t6105 = t3400 * t6068;
    (t6092, t6094, t6096, t6098, t6100, t6102, t6104, t6105)
}
