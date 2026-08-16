//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1842/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1842(t22986: f64, t25192: f64, t82159: f64, t254: f64, t853: f64, t23164: f64, t23204: f64, t25341: f64, t12971: f64, t6552: f64, t6553: f64, t6554: f64) -> (f64, f64, f64, f64) {
    let t87010 = t22986 * t82159 * t25192;
    let t87013 = t853 * t254;
    let t87028 = t23164 * t23204 * t25341;
    let t87033 = t6552 * t6553 * t6554 * t12971;
    (t87010, t87013, t87028, t87033)
}
