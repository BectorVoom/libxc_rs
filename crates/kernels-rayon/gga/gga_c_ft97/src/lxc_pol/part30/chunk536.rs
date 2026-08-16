//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 536/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk536(t5995: f64, t92: f64, t1426: f64, t2399: f64, t6067: f64, t681: f64, t6063: f64, t6008: f64, t683: f64) -> (f64, f64, f64, f64, f64) {
    let t24204 = t5995 * t92;
    let t24211 = t2399 * t1426;
    let t24220 = t681 * t6067;
    let t24223 = t681 * t6063;
    let t24231 = t683 * t6008;
    (t24204, t24211, t24220, t24223, t24231)
}
