//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 34/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk34(t43: f64, t50: f64, t45: f64, t47: f64, t52: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t44 = t43 <= zeta_threshold;
    let t51 = t50 <= zeta_threshold;
    let t97 = t45 * t45;
    let t98 = t47 * t47;
    let t99 = piecewise3(t44, t97, t98);
    let t100 = t52 * t52;
    let t101 = piecewise3(t51, t97, t100);
    let t103 = t99 / 2.0_f64 + t101 / 2.0_f64;
    (t98, t100, t103)
}
