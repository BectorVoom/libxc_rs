//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1221/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1221(t30559: f64, t30561: f64, t34396: f64, t34400: f64, t34413: f64, t34414: f64, t34417: f64, t37069: f64, t37076: f64, t39273: f64, t39275: f64, t39277: f64, t39279: f64, t39281: f64, t39283: f64, t39285: f64, t39292: f64, t39294: f64) -> f64 {
    let t41568 = 0.85748036236139473947e-3_f64 * t39273 - 0.85748036236139473944e-3_f64 * t39275 + 0.94344276868812456207e-3_f64 * t39277 - 0.13719685797782315831e-1_f64 * t39279 + 0.32012600194825403606e-1_f64 * t34396 + 0.13719685797782315831e-1_f64 * t39281 + 0.13719685797782315831e-1_f64 * t39283 - t37069 - 0.68598428988911579156e-2_f64 * t34400 - 0.16006300097412701803e-1_f64 * t39285 - t37076 + t34413 - t34414 - t34417 + 0.83861579438944405516e-3_f64 * t30559 + 0.20965394859736101379e-2_f64 * t30561 - 0.916875e-1_f64 * t39292 - 0.183375e0_f64 * t39294;
    t41568
}
