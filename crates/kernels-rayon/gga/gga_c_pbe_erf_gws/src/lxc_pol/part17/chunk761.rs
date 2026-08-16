//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 761/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk761(t191: f64, t5263: f64, t4939: f64, t1740: f64, t579: f64, t1867: f64, t582: f64, t185: f64, t1660: f64, t9: f64, t1665: f64, t587: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5264 = t191 * t5263;
    let t5271 = 0.11197407407407407407e0_f64 * t4939;
    let t5278 = t579 * t1740;
    let t5280 = t582 * t1867;
    let t5281 = t185 * t5280;
    let t5283 = t9 * t1660;
    let t5284 = t5283 * t1665;
    let t5285 = t587 * t5284;
    (t5264, t5271, t5278, t5281, t5283, t5285)
}
