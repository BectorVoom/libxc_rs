//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 62/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk62(t149: f64, t17: f64, t19: f64, t27: f64) -> (f64, f64, f64, f64, f64) {
    let t175 = 0.1898925e1_f64 * t149 + t17 + t19 + t27;
    let t178 = 1.0_f64 + 0.16081979498692535067e2_f64 / t175;
    let t179 = f64::ln(t178);
    let t180 = 0.1328816518e-1_f64 * t179;
    let t181 = t175 * t175;
    let t182 = 1.0_f64 / t181;
    (t175, t178, t180, t181, t182)
}
