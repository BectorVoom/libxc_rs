//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 663/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk663(t1208: f64, t218: f64, t811: f64, t25057: f64, t820: f64, t1196: f64, t1472: f64, t27720: f64, t2691: f64, t7005: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t28628 = t218 * t1208;
    let t28629 = t28628 * t811;
    let t28630 = t25057 * t28629;
    let t28633 = t28628 * t820;
    let t28634 = t25057 * t28633;
    let t28637 = t218 * t1196;
    let t28638 = t28637 * t820;
    let t28639 = t25057 * t28638;
    let t28646 = t1472 * t27720;
    let t28652 = t2691 * t7005;
    (t28629, t28630, t28633, t28634, t28638, t28639, t28646, t28652)
}
