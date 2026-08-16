//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1902/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1902(t25306: f64, t6637: f64, t6552: f64, t23168: f64, t7521: f64, t4119: f64, t6638: f64, t22893: f64, t7520: f64, t23164: f64, t1519: f64, t234: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t25307 = t6637 * t25306;
    let t25308 = t6552 * t25307;
    let t25310 = t23168 * t7521;
    let t25312 = t6638 * t4119;
    let t25313 = t6637 * t25312;
    let t25314 = t6552 * t25313;
    let t25316 = t22893 * t7520;
    let t25317 = t23164 * t25316;
    let t25319 = t234 * t1519;
    (t25307, t25308, t25310, t25312, t25313, t25314, t25316, t25317, t25319)
}
