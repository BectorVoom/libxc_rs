//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 892/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk892(t25: f64, t1268: f64, t2312: f64, t2314: f64, t2319: f64, t2363: f64, t671: f64, t88: f64, t526: f64, t606: f64, t2249: f64, t514: f64, t528: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64) {
    let t26 = t25 <= zeta_threshold;
    let t3660 = 2.0_f64 * t1268 * t2363 + 4.0_f64 * t2314 * t671 + 2.0_f64 * t2319 * t88 + t2312;
    let t3664 = 1.0_f64 / t526;
    let t3665 = t606 * t606;
    let t3671 = piecewise3(t26, 0.0_f64, 4.0_f64 / 9.0_f64 * t3664 * t3665 + 4.0_f64 / 3.0_f64 * t514 * t2249);
    let t3672 = 1.0_f64 / t528;
    (t3660, t3664, t3665, t3671, t3672)
}
