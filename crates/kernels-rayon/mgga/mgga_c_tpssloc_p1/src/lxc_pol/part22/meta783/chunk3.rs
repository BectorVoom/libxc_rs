//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2682/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2682(t20582: f64, t40021: f64, t118: f64, t20356: f64, t40412: f64, t794: f64, t20576: f64, t3726: f64, t16101: f64, t40372: f64, t40401: f64, t40402: f64, t40407: f64, t46838: f64, t56501: f64, t56505: f64, t56514: f64, t74389: f64) -> f64 {
    let t74741 = t40021 * t20582;
    let t74745 = t40412 * t118 * t794 * t20356;
    let t74747 = t3726 * t20576;
    let t74754 = 0.29999999999999999999e-1_f64 * t56501 - 0.14999999999999999999e-1_f64 * t56505 - 0.74999999999999999997e-2_f64 * t56514 + 0.27777777777777777778e-3_f64 * t40372 + 0.46666666666666666664e-1_f64 * t74741 + 0.99999999999999999995e-2_f64 * t74745 + 0.38888888888888888887e-2_f64 * t74747 - 0.59999999999999999996e-1_f64 * t16101 * t46838 * t74389 - t40401 + 0.56172839506172839504e-1_f64 * t40402 + 0.3287037037037037037e-1_f64 * t40407;
    t74754
}
