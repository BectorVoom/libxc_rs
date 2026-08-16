//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2358/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2358(t177: f64, t2495: f64, t2514: f64, t2537: f64, t2539: f64, t2548: f64, t2554: f64, t2556: f64, t2557: f64, t2597: f64, t2598: f64, t2604: f64, t39419: f64, t39422: f64, t39483: f64, t39520: f64, t39528: f64, t39531: f64, t39871: f64, t39875: f64, t39886: f64, t39894: f64, t39909: f64, t729: f64, t730: f64, t731: f64, t739: f64, t745: f64, t9371: f64, t9433: f64, t9446: f64, t9536: f64) -> f64 {
    let t39913 = 36.0_f64 * t2554 * t2539 * t2548 - 0.11579025239058625248e4_f64 * t9433 * t2557 * t2548 - 8.0_f64 * t2537 * t731 * t9446 + t39419 + t39422 - 0.35089341735807877242e1_f64 * t2597 * t39871 * t745 + 0.6233709278045326953e3_f64 * t9536 * t39875 * t2495 + 0.12865583598954028054e3_f64 * t2554 * t9446 * t2556 * t729 + 0.21053605041484726346e2_f64 * t2604 * t2598 * t2514 + t39483 - t39520 - 6.0_f64 * t2537 * t39886 * t730 + t39528 + 0.51947577317044391277e2_f64 * t2604 * t39871 * t2495 - 0.12304822629859687989e5_f64 * t177 * t39894 * t39875 * t9371 + 0.5848223622634646207e0_f64 * t739 * t39909 * t745 - t39531;
    t39913
}
