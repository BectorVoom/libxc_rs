//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1105/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1105(t7447: f64, t9663: f64, t7440: f64, t9734: f64, t31773: f64, t9660: f64, t9730: f64, t30569: f64, t30570: f64, t30577: f64, t30582: f64, t34422: f64, t34430: f64, t37093: f64, t39299: f64, t39302: f64, t39305: f64, t39308: f64, t39311: f64, t39314: f64, t39318: f64) -> f64 {
    let t39320 = t7447 * t9663;
    let t39322 = t7440 * t9734;
    let t39324 = t31773 * t9660;
    let t39326 = t7447 * t9730;
    let t39328 = -t34422 - 5.0_f64 / 32.0_f64 * t39299 - t39302 / 32.0_f64 + t39305 / 16.0_f64 + t39308 / 64.0_f64 + t39311 / 64.0_f64 - t39314 / 64.0_f64 - t30569 - t34430 - 0.94344276868812456205e-2_f64 * t30570 + t30577 + 0.62896184579208304134e-3_f64 * t30582 - t37093 + t39318 / 48.0_f64 - 0.84046875e-1_f64 * t39320 + 0.84046875e-1_f64 * t39322 + 0.16809375e0_f64 * t39324 - 11.0_f64 / 192.0_f64 * t39326;
    t39328
}
