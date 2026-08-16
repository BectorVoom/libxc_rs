//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 499/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk499(t1413: f64, t1449: f64, t2481: f64, t2484: f64, t2507: f64, t2510: f64, t2513: f64, t2528: f64, t430: f64, t453: f64, t987: f64, t1512: f64) -> (f64, f64) {
    let t2531 = 0.165625e-1_f64 * t2481 * t987 - 0.33125e-1_f64 * t1413 * t2484 + 0.165625e-1_f64 * t430 * t2507 - 0.33125e-1_f64 * t1413 * t2510 + 0.496875e-1_f64 * t1449 * t2513 - 0.165625e-1_f64 * t453 * t2528;
    let t2535 = 0.5848223622634646207e0_f64 * t1512;
    (t2531, t2535)
}
