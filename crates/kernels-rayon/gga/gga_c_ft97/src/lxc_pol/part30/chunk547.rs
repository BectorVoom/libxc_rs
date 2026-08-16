//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 547/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk547(t91: f64, t9890: f64, t6109: f64, t6111: f64, t681: f64, t1434: f64, t6124: f64, t1439: f64, t1636: f64, t89: f64, t375: f64, t6144: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t24447 = t91 * t9890;
    let t24455 = t6109 * t681 * t6111;
    let t24470 = t1434 * t681 * t6124;
    let t24482 = t89 * t1636 * t1439;
    let t24483 = 4.0_f64 / 9.0_f64 * t24482;
    let t24485 = t89 * t375 * t6144;
    (t24447, t24455, t24470, t24482, t24483, t24485)
}
