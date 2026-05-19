//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1218/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1218<F: Float>(t32435: F, t34309: F, t34311: F, t34315: F, t34317: F, t34332: F, t34333: F, t34336: F, t34338: F, t34339: F, t37034: F, t37036: F, t39182: F, t39186: F, t39189: F, t39192: F, t39194: F, t39203: F) -> F {
    let t41523 = -F::cast_from(0.21437009059034868486e-2_f64) * t39182 - F::cast_from(0.21437009059034868486e-2_f64) * t39186 - F::cast_from(0.14291339372689912324e-2_f64) * t39189 + F::cast_from(0.16006300097412701803e-1_f64) * t34309 + F::cast_from(0.51448821741683684367e-2_f64) * t34311 + F::cast_from(0.34299214494455789578e-2_f64) * t34315 + F::cast_from(0.51448821741683684367e-2_f64) * t34317 + F::cast_from(0.34299214494455789578e-1_f64) * t39192 - F::cast_from(0.13719685797782315831e-1_f64) * t39194 - t34332 - t34333 + t32435 + F::cast_from(0.12579236915841660828e-2_f64) * t34336 + t34338 + t34339 + t37034 + F::cast_from(0.62896184579208304138e-3_f64) * t39203 - t37036;
    t41523
}
