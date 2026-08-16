//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 644/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk644(t487: f64, t6557: f64, t379: f64, t1909: f64, t23327: f64, t3200: f64, t1901: f64, t26230: f64, t26234: f64, t26237: f64, t26242: f64, t26246: f64, t26249: f64, t26252: f64, t26255: f64, t26259: f64, t26262: f64, t26265: f64, t446: f64) -> (f64, f64) {
    let t26267 = t487 * t6557;
    let t26268 = t26267 * t379;
    let t26269 = t1909 * t26268;
    let t26272 = t23327 * t3200;
    let t26275 = 2.0_f64 / 3.0_f64 * t446 * t26230 + t446 * t26234 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t26237 + t446 * t26242 / 3.0_f64 + t1901 * t26246 / 9.0_f64 + t1901 * t26249 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t26252 + 2.0_f64 / 3.0_f64 * t446 * t26255 + 2.0_f64 / 3.0_f64 * t446 * t26259 + 2.0_f64 / 3.0_f64 * t446 * t26262 - t26265 / 27.0_f64 + t1901 * t26269 / 9.0_f64 + t1901 * t26272 / 9.0_f64;
    (t26268, t26275)
}
