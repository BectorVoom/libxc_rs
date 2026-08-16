//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 581/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk581(t2460: f64, t2515: f64, t141: f64, t2465: f64, t861: f64, t2469: f64, t2455: f64, t2462: f64, t2467: f64, t2471: f64, t2489: f64, t2497: f64, t2499: f64, t2505: f64, t2507: f64, t2512: f64, t2513: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2516 = t2515 * t2460;
    let t2517 = t141 * t2516;
    let t2519 = t861 * t2465;
    let t2520 = t141 * t2519;
    let t2522 = t861 * t2469;
    let t2523 = t141 * t2522;
    let t2525 = -0.9494625e0_f64 * t2489 + 0.1898925e1_f64 * t2497 + t2499 + 0.19931111111111111111e0_f64 * t2455 - 0.19931111111111111111e0_f64 * t2462 + 0.59793333333333333334e0_f64 * t2467 - 0.29896666666666666667e0_f64 * t2471 + 0.15358125e0_f64 * t2505 + 0.3071625e0_f64 * t2507 + t2512 + 0.10954222222222222222e0_f64 * t2513 - 0.27385555555555555556e-1_f64 * t2517 + 0.16431333333333333333e0_f64 * t2520 - 0.82156666666666666667e-1_f64 * t2523;
    (t2516, t2517, t2519, t2520, t2522, t2523, t2525)
}
