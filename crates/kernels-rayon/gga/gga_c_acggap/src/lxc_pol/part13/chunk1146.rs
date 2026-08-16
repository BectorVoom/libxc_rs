//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1146/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1146(t2001: f64, t4552: f64, t1998: f64, t5089: f64, t1451: f64, t7605: f64, t1423: f64, t7736: f64, t30318: f64, t542: f64, t4886: f64, t2327: f64, t7630: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t35731 = t2001 * t4552;
    let t35733 = t1998 * t5089;
    let t35736 = t7605 * t1451;
    let t35737 = 0.34299214494455789578e-2_f64 * t35736;
    let t35738 = t7736 * t1423;
    let t35740 = t30318 * t542;
    let t35742 = t2001 * t4886;
    let t35744 = t7630 * t2327;
    (t35731, t35733, t35737, t35738, t35740, t35742, t35744)
}
