//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 911/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk911(t582: f64, t9439: f64, t167: f64, t40465: f64, t2101: f64, t3578: f64, t40424: f64, t3018: f64, t63: f64, t373: f64, t397: f64, t370: f64, t971: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t50558 = t582 * t9439;
    let t50744 = t40465 * t167;
    let t50773 = t2101 * t3578;
    let t51151 = t40424 * t167;
    let t58585 = t3018 * t63;
    let t58607 = t397 * t373;
    let t59631 = t370 * t971;
    (t50558, t50744, t50773, t51151, t58585, t58607, t59631)
}
