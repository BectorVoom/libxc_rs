//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 215/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk215(t122: f64, t1593: f64, t35: f64, t63: f64, t37: f64, t62: f64) -> (f64, f64, f64, f64) {
    let t1594 = t122 * t1593;
    let t1597 = t35 * t35;
    let t1598 = t1597 * t63;
    let t1602 = t37 * t62;
    (t1594, t1597, t1598, t1602)
}
