//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2673/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2673(t39365: f64, t19681: f64, t2371: f64, t54380: f64, t54382: f64, t39374: f64, t39387: f64, t20067: f64, t3719: f64, t3918: f64, t39360: f64, t39364: f64, t39373: f64, t39384: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t56167 = 0.11393789434848516922e-2_f64 * t39365;
    let t56168 = t19681 * t2371;
    let t56169 = 0.11696447245269292414e1_f64 * t56168;
    let t56170 = 0.32530743900905219526e-1_f64 * t54380;
    let t56171 = 0.96319466275353142155e0_f64 * t54382;
    let t56172 = 0.20508037716432813316e4_f64 * t39374;
    let t56173 = 0.5848223622634646207e0_f64 * t39387;
    let t56174 = 3.0_f64 * t20067 * t3719 * t3918 + t39360 + t39364 + t39373 - t39384 - t56167 + t56169 + t56170 + t56171 - t56172 - t56173;
    (t56167, t56169, t56170, t56171, t56172, t56173, t56174)
}
