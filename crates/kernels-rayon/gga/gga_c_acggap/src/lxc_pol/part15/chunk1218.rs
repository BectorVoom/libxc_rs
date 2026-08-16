//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1218/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1218(t32435: f64, t34309: f64, t34311: f64, t34315: f64, t34317: f64, t34332: f64, t34333: f64, t34336: f64, t34338: f64, t34339: f64, t37034: f64, t37036: f64, t39182: f64, t39186: f64, t39189: f64, t39192: f64, t39194: f64, t39203: f64) -> f64 {
    let t41523 = -0.21437009059034868486e-2_f64 * t39182 - 0.21437009059034868486e-2_f64 * t39186 - 0.14291339372689912324e-2_f64 * t39189 + 0.16006300097412701803e-1_f64 * t34309 + 0.51448821741683684367e-2_f64 * t34311 + 0.34299214494455789578e-2_f64 * t34315 + 0.51448821741683684367e-2_f64 * t34317 + 0.34299214494455789578e-1_f64 * t39192 - 0.13719685797782315831e-1_f64 * t39194 - t34332 - t34333 + t32435 + 0.12579236915841660828e-2_f64 * t34336 + t34338 + t34339 + t37034 + 0.62896184579208304138e-3_f64 * t39203 - t37036;
    t41523
}
