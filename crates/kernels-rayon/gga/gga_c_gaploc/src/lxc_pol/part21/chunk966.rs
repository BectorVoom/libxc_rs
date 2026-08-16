//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 966/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk966(t10770: f64, t2508: f64, t2927: f64, t954: f64, t3448: f64, t7137: f64, t8440: f64, t977: f64, t2728: f64, t2969: f64, t3459: f64, t841: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10772 = 0.76905262301422242837e-2_f64 * t2508 * t10770;
    let t10773 = t954 * t2927;
    let t10775 = 0.76905262301422242837e-2_f64 * t2508 * t10773;
    let t10788 = 0.20508069947045931423e-1_f64 * t7137 * t3448;
    let t10797 = t8440 * t977;
    let t10798 = t2969 * t2728;
    let t10802 = t3459 * t841;
    (t10772, t10773, t10775, t10788, t10797, t10798, t10802)
}
