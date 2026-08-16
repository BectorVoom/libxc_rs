//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 968/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk968(t1466: f64, t34261: f64, t681: f64, t33993: f64, t870: f64, t2842: f64, t7662: f64, t2399: f64, t7617: f64, t34333: f64, t6210: f64, t458: f64, t7580: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t142995 = t1466 * t681 * t34261;
    let t142999 = t33993 * t870;
    let t143002 = t7662 * t2842;
    let t143007 = 4.0_f64 / 27.0_f64 * t1466 * t2399 * t7617;
    let t143008 = t6210 * t34333;
    let t143017 = t7580 * t458;
    (t142995, t142999, t143002, t143007, t143008, t143017)
}
