//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1164/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1164(t42452: f64, t22084: f64, t22590: f64, t22592: f64, t33523: f64, t22594: f64, t33527: f64, t33530: f64, t22599: f64, t18527: f64, t18529: f64, t18556: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t48478 = 16.0_f64 * t42452;
    let t48479 = 0.14035736153892489771e2_f64 * t22084;
    let t48480 = 48.0_f64 * t22590;
    let t48481 = 96.0_f64 * t22592;
    let t48482 = 0.35089340384731224426e1_f64 * t33523;
    let t48483 = 0.14035736153892489771e2_f64 * t22594;
    let t48484 = 48.0_f64 * t33527;
    let t48485 = 0.14649244029402527953e-2_f64 * t33530;
    let t48486 = 0.22787712934626154593e-2_f64 * t22599;
    let t48487 = -t48478 + t48479 + t18527 - t18529 - t48480 - t48481 - t48482 - t48483 - t48484 + t48485 - t48486 - t18556;
    (t48478, t48479, t48480, t48481, t48482, t48483, t48484, t48485, t48486, t48487)
}
