//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 761/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk761(t2661: f64, t89: f64, t9725: f64, t2724: f64, t811: f64, t2719: f64, t816: f64, t820: f64, t272: f64, t9606: f64, t9525: f64, t2697: f64, t688: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10286 = t89 * t9725 * t2661;
    let t10292 = t2724 * t811;
    let t10296 = t816 * t2719;
    let t10297 = t10296 * t820;
    let t10304 = 1.0_f64 / t272 / t9606;
    let t10305 = t10304 * t9525;
    let t10308 = t2697 * t688;
    (t10286, t10292, t10296, t10297, t10304, t10305, t10308)
}
