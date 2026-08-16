//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 588/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk588(t2754: f64, t2787: f64, t2343: f64, t1437: f64, t3565: f64, t2765: f64, t2792: f64, t3531: f64, t535: f64, t3529: f64, t599: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11241 = t2787 * t2754;
    let t11242 = t2343 * t11241;
    let t11245 = t3565 * t1437;
    let t11248 = t2765 * t2792;
    let t11251 = t535 * t3531;
    let t11254 = t599 * t3529;
    (t11241, t11242, t11245, t11248, t11251, t11254)
}
