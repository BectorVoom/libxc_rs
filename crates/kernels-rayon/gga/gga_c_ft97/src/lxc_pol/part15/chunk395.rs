//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 395/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk395(t408: f64, t938: f64, t64: f64, t66: f64, t1710: f64, t173: f64, t934: f64, t419: f64, t1736: f64, t420: f64) -> (f64, f64, f64, f64, f64) {
    let t3070 = t408 * t938;
    let t3076 = t64 * t66;
    let t3077 = t1710 * t938;
    let t3085 = t173 * t934;
    let t3086 = t419 * t3085;
    let t3088 = t420 * t1736;
    (t3070, t3076, t3077, t3086, t3088)
}
