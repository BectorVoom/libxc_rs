//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 662/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk662(t165: f64, t6615: f64, t379: f64, t1969: f64, t376: f64, t6617: f64, t23997: f64, t3483: f64, t16658: f64, t2: f64, t4: f64, t26: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t26567 = t6615 * t165;
    let t26568 = t26567 * t379;
    let t26569 = t1969 * t26568;
    let t26574 = t376 * t6617;
    let t26577 = t23997 * t3483;
    let t26579 = t16658 * t2;
    let t26580 = t26579 * t4;
    let t26581 = t26580 * t26;
    (t26567, t26568, t26569, t26574, t26577, t26579, t26581)
}
