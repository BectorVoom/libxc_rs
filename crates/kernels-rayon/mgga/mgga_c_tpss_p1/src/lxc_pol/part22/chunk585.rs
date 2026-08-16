//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 585/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk585(t2551: f64, t885: f64, t2453: f64, t2511: f64, t2455: f64, t2462: f64, t2467: f64, t2471: f64, t2489: f64, t2497: f64, t2505: f64, t2507: f64, t2513: f64, t2517: f64, t2520: f64, t2523: f64) -> (f64, f64, f64, f64) {
    let t2552 = t2551 * t885;
    let t2557 = 0.68863333333333333333e0_f64 * t2453;
    let t2564 = 0.17365833333333333333e0_f64 * t2511;
    let t2569 = -0.17648625e1_f64 * t2489 + 0.3529725e1_f64 * t2497 + t2557 + 0.34431666666666666666e0_f64 * t2455 - 0.34431666666666666667e0_f64 * t2462 + 0.103295e1_f64 * t2467 - 0.516475e0_f64 * t2471 + 0.31558125e0_f64 * t2505 + 0.6311625e0_f64 * t2507 + t2564 + 0.13892666666666666667e0_f64 * t2513 - 0.34731666666666666667e-1_f64 * t2517 + 0.20839e0_f64 * t2520 - 0.104195e0_f64 * t2523;
    (t2552, t2557, t2564, t2569)
}
