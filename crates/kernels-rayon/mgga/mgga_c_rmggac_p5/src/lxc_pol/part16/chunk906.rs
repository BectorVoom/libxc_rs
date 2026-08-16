//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 906/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk906(t40658: f64, t9222: f64, t39171: f64, t8571: f64, t1970: f64, t236: f64, t498: f64, t6172: f64, t7231: f64, t321: f64, t3352: f64, t1971: f64, t333: f64, t511: f64) -> (f64, f64, f64, f64, f64) {
    let t45062 = t9222 * t40658;
    let t45064 = t8571 * t39171;
    let t45069 = t1970 * t7231 * t236 * t6172 * t498;
    let t45074 = t1970 * t3352 * t236 * t6172 * t321;
    let t45080 = t1970 * t1971 * t511 * t6172 * t333;
    (t45062, t45064, t45069, t45074, t45080)
}
