//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2358/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2358<F: Float>(t177: F, t2495: F, t2514: F, t2537: F, t2539: F, t2548: F, t2554: F, t2556: F, t2557: F, t2597: F, t2598: F, t2604: F, t39419: F, t39422: F, t39483: F, t39520: F, t39528: F, t39531: F, t39871: F, t39875: F, t39886: F, t39894: F, t39909: F, t729: F, t730: F, t731: F, t739: F, t745: F, t9371: F, t9433: F, t9446: F, t9536: F) -> F {
    let t39913 = F::new(36.0) * t2554 * t2539 * t2548 - F::cast_from(0.11579025239058625248e4_f64) * t9433 * t2557 * t2548 - F::new(8.0) * t2537 * t731 * t9446 + t39419 + t39422 - F::cast_from(0.35089341735807877242e1_f64) * t2597 * t39871 * t745 + F::cast_from(0.6233709278045326953e3_f64) * t9536 * t39875 * t2495 + F::cast_from(0.12865583598954028054e3_f64) * t2554 * t9446 * t2556 * t729 + F::cast_from(0.21053605041484726346e2_f64) * t2604 * t2598 * t2514 + t39483 - t39520 - F::new(6.0) * t2537 * t39886 * t730 + t39528 + F::cast_from(0.51947577317044391277e2_f64) * t2604 * t39871 * t2495 - F::cast_from(0.12304822629859687989e5_f64) * t177 * t39894 * t39875 * t9371 + F::cast_from(0.5848223622634646207e0_f64) * t739 * t39909 * t745 - t39531;
    t39913
}
