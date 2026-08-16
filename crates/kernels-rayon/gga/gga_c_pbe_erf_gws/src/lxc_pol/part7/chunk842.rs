//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 842/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk842(t2182: f64, t274: f64, t810: f64, t824: f64, t821: f64, t3259: f64, t814: f64, t2264: f64, t899: f64, t923: f64, t6636: f64, t6684: f64) -> (f64, f64, f64, f64, f64) {
    let t9488 = t274 * t2182;
    let t9504 = t824 * t810;
    let t9505 = t821 * t9504;
    let t9568 = t3259 * t814;
    let t9630 = t899 * t2264 * t923;
    let t9637 = t6684 * t6636;
    (t9488, t9505, t9568, t9630, t9637)
}
