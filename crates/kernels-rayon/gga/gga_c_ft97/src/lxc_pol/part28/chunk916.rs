//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 916/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk916(t37939: f64, t409: f64, t1293: f64, t8051: f64, t1710: f64, t5532: f64, t1598: f64, t1711: f64, t22511: f64, t22817: f64, t3076: f64, t1669: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t92642 = t37939 * t409;
    let t92809 = t8051 * t1293;
    let t92920 = t1710 * t5532;
    let t93014 = t1598 * t1711;
    let t93046 = t22817 * t22511;
    let t93047 = t3076 * t93046;
    let t93117 = t1669 * t93046;
    (t92642, t92809, t92920, t93014, t93047, t93117)
}
