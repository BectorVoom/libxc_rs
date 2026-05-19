//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1221/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1221<F: Float>(t30559: F, t30561: F, t34396: F, t34400: F, t34413: F, t34414: F, t34417: F, t37069: F, t37076: F, t39273: F, t39275: F, t39277: F, t39279: F, t39281: F, t39283: F, t39285: F, t39292: F, t39294: F) -> F {
    let t41568 = F::cast_from(0.85748036236139473947e-3_f64) * t39273 - F::cast_from(0.85748036236139473944e-3_f64) * t39275 + F::cast_from(0.94344276868812456207e-3_f64) * t39277 - F::cast_from(0.13719685797782315831e-1_f64) * t39279 + F::cast_from(0.32012600194825403606e-1_f64) * t34396 + F::cast_from(0.13719685797782315831e-1_f64) * t39281 + F::cast_from(0.13719685797782315831e-1_f64) * t39283 - t37069 - F::cast_from(0.68598428988911579156e-2_f64) * t34400 - F::cast_from(0.16006300097412701803e-1_f64) * t39285 - t37076 + t34413 - t34414 - t34417 + F::cast_from(0.83861579438944405516e-3_f64) * t30559 + F::cast_from(0.20965394859736101379e-2_f64) * t30561 - F::new(0.916875e-1) * t39292 - F::new(0.183375e0) * t39294;
    t41568
}
