//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1175/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1175(t31697: f64, t31702: f64, t31704: f64, t31721: f64, t36063: f64, t36066: f64, t36068: f64, t36070: f64, t36072: f64, t36075: f64, t36077: f64, t36082: f64, t36083: f64, t36086: f64, t36088: f64, t36090: f64, t36093: f64) -> f64 {
    let t36095 = t36063 / 48.0_f64 - t36066 + t36068 / 64.0_f64 + t36070 + 0.53592522647587171215e-3_f64 * t31697 - t36072 + 0.31448092289604152068e-3_f64 * t31702 + 0.41930789719472202756e-3_f64 * t31704 + t36075 + 0.18868855373762491241e-2_f64 * t36077 + t36082 - t31721 + 0.21437009059034868486e-3_f64 * t36083 + t36086 + t36088 - t36090 - 0.47172138434406228102e-3_f64 * t36093;
    t36095
}
