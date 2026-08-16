//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1123/1303 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1123(t300: f64, t5769: f64, t961: f64, t2904: f64, t5790: f64, t952: f64, t959: f64, t14473: f64, t1589: f64, t4483: f64, t4493: f64, t4489: f64) -> (f64, f64, f64, f64, f64) {
    let t17934 = t300 * t5769;
    let t17936 = 0.5848223622634646207e0_f64 * t17934 * t961;
    let t17937 = t2904 * t5790;
    let t17938 = t17937 * t952;
    let t17940 = 0.11696447245269292414e1_f64 * t959 * t17938;
    let t17942 = 0.11696447245269292414e1_f64 * t14473 * t1589;
    let t17944 = 0.11696447245269292414e1_f64 * t4483 * t4493;
    let t17946 = 0.23392894490538584828e1_f64 * t4483 * t4489;
    (t17936, t17940, t17942, t17944, t17946)
}
