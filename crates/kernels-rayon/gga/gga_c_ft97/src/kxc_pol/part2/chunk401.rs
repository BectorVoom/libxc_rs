//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 401/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk401(t358: f64, t604: f64, t363: f64, t609: f64, t2210: f64, t1647: f64, t167: f64, t569: f64, t157: f64, t2101: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2211 = t604 * t358;
    let t2212 = t363 * t609;
    let t2213 = t2211 * t2212;
    let t2214 = t2210 * t2213;
    let t2218 = t569 * t167 * t1647;
    let t2221 = t2101 * t157;
    (t2211, t2212, t2213, t2214, t2218, t2221)
}
