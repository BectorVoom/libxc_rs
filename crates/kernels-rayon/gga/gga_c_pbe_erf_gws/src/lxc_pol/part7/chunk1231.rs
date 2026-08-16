//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1231/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1231(t19894: f64, t3074: f64, t6170: f64, t840: f64, t2353: f64, t353: f64, t814: f64, t859: f64, t2231: f64, t810: f64, t8599: f64, t4386: f64) -> (f64, f64, f64, f64, f64) {
    let t21727 = t3074 * t19894;
    let t21733 = t840 * t6170;
    let t21737 = t859 * t353 * t2353 * t814;
    let t21742 = t8599 * t353 * t2231 * t810;
    let t21747 = t4386 * t353 * t2353 * t810;
    (t21727, t21733, t21737, t21742, t21747)
}
