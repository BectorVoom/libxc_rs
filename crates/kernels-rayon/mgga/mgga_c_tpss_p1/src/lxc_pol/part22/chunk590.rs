//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 590/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk590(t2595: f64, t904: f64, t2453: f64, t2511: f64, t2455: f64, t2462: f64, t2467: f64, t2471: f64, t2489: f64, t2497: f64, t2505: f64, t2507: f64, t2513: f64, t2517: f64, t2520: f64, t2523: f64) -> (f64, f64, f64, f64) {
    let t2596 = t2595 * t904;
    let t2601 = 0.40256666666666666667e0_f64 * t2453;
    let t2608 = 0.137975e0_f64 * t2511;
    let t2613 = -0.1294625e1_f64 * t2489 + 0.258925e1_f64 * t2497 + t2601 + 0.20128333333333333334e0_f64 * t2455 - 0.20128333333333333333e0_f64 * t2462 + 0.60385e0_f64 * t2467 - 0.301925e0_f64 * t2471 + 0.82524375e-1_f64 * t2505 + 0.16504875e0_f64 * t2507 + t2608 + 0.11038e0_f64 * t2513 - 0.27595e-1_f64 * t2517 + 0.16557e0_f64 * t2520 - 0.82785e-1_f64 * t2523;
    (t2596, t2601, t2608, t2613)
}
