//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 1196/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk1196<F: Float>(t15979: F, t15982: F, t15984: F, t182: F, t19572: F, t16164: F, t12134: F, t12136: F, t12138: F, t12142: F, t12123: F, t12130: F, t12133: F, t12141: F, t16171: F, t9853: F, t9859: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t19689 = F::cast_from(0.48830526149350786811e-3_f64) * t15979;
    let t19690 = F::cast_from(40.0_f64) * t15982;
    let t19691 = F::cast_from(24.0_f64) * t15984;
    let t19693 = F::cast_from(0.19751673498613801407e-1_f64) * t19572 * t182;
    let t19694 = F::cast_from(0.23392894490538584828e1_f64) * t16164;
    let t19695 = F::cast_from(8.0_f64) * t12134;
    let t19696 = F::cast_from(20.0_f64) * t12136;
    let t19697 = F::cast_from(0.11696447245269292414e1_f64) * t12138;
    let t19698 = F::cast_from(0.5848223622634646207e0_f64) * t12142;
    let t19699 = t12123 + t19689 + t19690 - t19691 + t19693 + t12130 + t12133 + t19694 - t19695 + t19696 + t9853 + t19697 - t16171 + t9859 - t12141 - t19698;
    (t19689, t19690, t19691, t19693, t19694, t19695, t19696, t19697, t19698, t19699)
}
