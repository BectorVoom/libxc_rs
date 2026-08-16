//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 703/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk703(t171: f64, t4562: f64, t1355: f64, t169: f64, t700: f64, t1383: f64, t770: f64, t289: f64, t4598: f64, t274: f64, t413: f64, t39: f64, t745: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5718 = t171 * t4562;
    let t5723 = t169 * t1355 * t700;
    let t5726 = t169 * t770 * t1383;
    let t5730 = 0.31835665774679373271e-1_f64 * t169 * t289 * t4598;
    let t5732 = 0.12798016258123051272e1_f64 * t413 * t274;
    let t5733 = t39 * t745;
    (t5718, t5723, t5726, t5730, t5732, t5733)
}
