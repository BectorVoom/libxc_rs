//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 943/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk943(t33350: f64, t695: f64, t141116: f64, t2387: f64, t2917: f64, t36791: f64, t108517: f64, t141111: f64, t1636: f64, t7528: f64, t89: f64, t7532: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t141166 = t33350 * t695;
    let t141171 = t2387 * t141116;
    let t141172 = t36791 * t2917;
    let t141176 = t108517 * t141111;
    let t141200 = t89 * t1636 * t7528;
    let t141201 = 8.0_f64 / 9.0_f64 * t141200;
    let t141203 = t89 * t1636 * t7532;
    (t141166, t141171, t141172, t141176, t141200, t141201, t141203)
}
