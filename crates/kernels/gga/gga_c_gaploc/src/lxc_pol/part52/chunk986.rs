//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 986/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk986<F: Float>(t1029: F, t12255: F, t14373: F, t14415: F, t14428: F, t14431: F, t1897: F, t1901: F, t2508: F, t2580: F, t2958: F, t3009: F, t38907: F, t39403: F, t43042: F, t44828: F, t44829: F, t44837: F, t44883: F, t44887: F, t44895: F, t47652: F, t50051: F, t50150: F, t702: F, t7129: F, t7137: F, t7226: F, t8670: F) -> F {
    let t50407 = t44828 - F::cast_from(0.85450291446024714264e-3_f64) * t44829 + F::cast_from(0.64087718584518535698e-3_f64) * t44837 + t44883 - t44887 + F::cast_from(0.41016139894091862845e-1_f64) * t7137 * t14415 - F::cast_from(0.23071578690426672851e-1_f64) * t2508 * t1901 * t50150 + F::cast_from(0.15381052460284448567e-1_f64) * t2508 * t2580 * t50051 + F::cast_from(0.38452631150711121419e-2_f64) * t43042 + F::cast_from(0.46143157380853345702e-1_f64) * t1897 * t12255 * t8670 + F::cast_from(0.18457262952341338281e0_f64) * t2508 * t2580 * t2958 * t38907 - F::cast_from(0.92286314761706691402e-1_f64) * t2508 * t7226 * t3009 * t38907 - F::cast_from(0.76905262301422242837e-2_f64) * t1897 * t14373 * t702 + F::cast_from(0.20508069947045931423e-1_f64) * t7137 * t14431 + F::cast_from(0.17090058289204942853e-2_f64) * t47652 - F::cast_from(0.46143157380853345702e-1_f64) * t7129 * t14428 - F::cast_from(0.46143157380853345702e-1_f64) * t2508 * t39403 * t1029 + t44895;
    t50407
}
