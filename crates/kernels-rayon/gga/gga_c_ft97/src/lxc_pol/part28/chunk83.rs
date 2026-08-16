//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 83/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk83(t118: f64, t29: f64, t174: f64, t170: f64, t173: f64) -> (f64, f64, f64, f64, f64) {
    let t177 = 0.469508e0_f64 * t118 + 0.4332925e0_f64 * t29;
    let t178 = t177 * t177;
    let t179 = 1.0_f64 / t178;
    let t180 = t174 * t179;
    let t184 = f64::exp(-t170 * t173 * t180 / 4.0_f64);
    (t177, t178, t179, t180, t184)
}
