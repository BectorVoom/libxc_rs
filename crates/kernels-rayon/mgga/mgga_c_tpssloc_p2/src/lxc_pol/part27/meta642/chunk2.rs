//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2187/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2187(t25192: f64, t81651: f64, t82074: f64, t225: f64, t25220: f64, t82259: f64, t6552: f64, t6555: f64, t87782: f64, t23270: f64, t25038: f64, t25191: f64, t87036: f64) -> (f64, f64, f64, f64, f64) {
    let t87835 = t81651 * t82074 * t25192;
    let t87836 = 0.16449340668482264365e-1_f64 * t87835;
    let t87837 = t25220 * t225;
    let t87847 = 0.12793931631041761173e0_f64 * t82259;
    let t87861 = t6552 * t87782 * t6555;
    let t87866 = t25038 * t23270 * t25191 * t87036;
    (t87836, t87837, t87847, t87861, t87866)
}
