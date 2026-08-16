//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 411/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk411(t149: f64, t165: f64, t1953: f64, t2081: f64, t2143: f64, t2158: f64, t2181: f64, t2228: f64, t2230: f64, t564: f64, t614: f64, t184: f64) -> (f64, f64) {
    let t2235 = -t149 * t2228 - t165 * t1953 - t165 * t2081 - 2.0_f64 * t564 * t614 - 4.0_f64 * t2143 - 2.0_f64 * t2158 + 4.0_f64 * t2181 + 2.0_f64 * t2230;
    let t2236 = t2235 * t184;
    (t2235, t2236)
}
