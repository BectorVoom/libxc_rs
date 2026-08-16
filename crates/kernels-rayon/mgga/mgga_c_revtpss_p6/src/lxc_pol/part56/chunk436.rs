//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 436/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk436(t2457: f64, t2501: f64, t2470: f64, t684: f64, t128: f64, t136: f64, t692: f64, t2435: f64, t2439: f64, t738: f64, t745: f64, t760: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2502 = t2501 * t2457;
    let t2504 = t684 * t2470;
    let t2507 = 1.0_f64/f64::sqrt(t128);
    let t2508 = t2507 * t136;
    let t2509 = t2508 * t2457;
    let t2511 = t692 * t2470;
    let t2514 = -0.57538888888888888889e0_f64 * t2502 + 0.11507777777777777778e1_f64 * t2504 + 0.40256666666666666667e0_f64 * t2435 + 0.366775e-1_f64 * t2509 + 0.73355e-1_f64 * t2511 + 0.137975e0_f64 * t2439;
    let t2516 = t738 * t2514 * t745;
    let t2518 = 0.5848223622634646207e0_f64 * t760 * t2516;
    (t2502, t2504, t2509, t2511, t2514, t2516, t2518)
}
