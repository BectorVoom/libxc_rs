//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1501/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1501(t15979: f64, t15982: f64, t15984: f64, t182: f64, t19572: f64, t16164: f64, t12134: f64, t12136: f64, t12138: f64, t12142: f64, t12123: f64, t12130: f64, t12133: f64, t12141: f64, t16171: f64, t9853: f64, t9859: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t19689 = 0.48830526149350786811e-3_f64 * t15979;
    let t19690 = 40.0_f64 * t15982;
    let t19691 = 24.0_f64 * t15984;
    let t19693 = 0.19751673498613801407e-1_f64 * t19572 * t182;
    let t19694 = 0.23392894490538584828e1_f64 * t16164;
    let t19695 = 8.0_f64 * t12134;
    let t19696 = 20.0_f64 * t12136;
    let t19697 = 0.11696447245269292414e1_f64 * t12138;
    let t19698 = 0.5848223622634646207e0_f64 * t12142;
    let t19699 = t12123 + t19689 + t19690 - t19691 + t19693 + t12130 + t12133 + t19694 - t19695 + t19696 + t9853 + t19697 - t16171 + t9859 - t12141 - t19698;
    (t19689, t19690, t19691, t19693, t19694, t19695, t19696, t19697, t19698, t19699)
}
