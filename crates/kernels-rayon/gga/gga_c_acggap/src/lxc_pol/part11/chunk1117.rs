//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1117/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1117(t1165: f64, t33751: f64, t604: f64, t7413: f64, t1181: f64, t30856: f64, t35324: f64, t599: f64, t31105: f64, t35287: f64, t35291: f64, t35294: f64, t35298: f64, t35302: f64, t35305: f64, t35308: f64, t35309: f64, t35311: f64, t35316: f64, t35318: f64, t35319: f64, t35321: f64, t35327: f64) -> f64 {
    let t35331 = t7413 * t1165 * t604 * t33751;
    let t35335 = t30856 * t1181 * t599 * t35324;
    let t35337 = 0.3773771074752498248e-2_f64 * t31105 - t35287 + t35291 + 0.62896184579208304136e-3_f64 * t35294 - 0.12862205435420921092e-2_f64 * t35298 + t35302 + 0.53592522647587171215e-3_f64 * t35305 - t35308 + 0.68598428988911579156e-2_f64 * t35309 + 0.34299214494455789578e-2_f64 * t35311 - t35316 - t35318 + 0.17149607247227894789e-2_f64 * t35319 - 0.68598428988911579156e-2_f64 * t35321 + 0.94344276868812456204e-3_f64 * t35327 - 0.94344276868812456204e-3_f64 * t35331 - 0.64311027177104605458e-3_f64 * t35335;
    t35337
}
