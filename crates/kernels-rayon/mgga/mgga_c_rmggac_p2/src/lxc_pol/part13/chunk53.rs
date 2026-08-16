//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 53/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk53(t4: f64, t140: f64, t34: f64, t6: f64, t12: f64, t13: f64, t138: f64, t135: f64, t77: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t141 = t4 * t4;
    let t142 = t140 * t141;
    let t145 = t142 * t6 / t34;
    let t147 = 0.379785e1_f64 * t13 + 0.8969e0_f64 * t12 + 0.204775e0_f64 * t138 + 0.123235e0_f64 * t145;
    let t150 = 1.0_f64 + 0.16081979498692535067e2_f64 / t147;
    let t151 = f64::ln(t150);
    let t153 = 0.621814e-1_f64 * t135 * t151;
    let t154 = 1.0_f64 / t77;
    (t141, t142, t145, t147, t150, t151, t153, t154)
}
