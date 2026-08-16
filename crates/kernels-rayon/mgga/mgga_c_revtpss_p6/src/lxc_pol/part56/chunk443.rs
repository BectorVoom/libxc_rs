//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 443/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk443(t675: f64, t738: f64, t182: f64, t737: f64, t177: f64, t2492: f64, t745: f64, t2514: f64, t2491: f64, t2495: f64, t123: f64, t173: f64, t186: f64, t2434: f64, t2522: f64, t2531: f64, t2537: f64, t2539: f64, t2549: f64, t2554: f64, t2557: f64, t2562: f64, t2569: f64, t2579: f64, t2587: f64, t268: f64, t724: f64, t731: f64, t739: f64, t746: f64) -> (f64, f64) {
    let t2591 = t675 * t738;
    let t2595 = t737 * t182;
    let t2596 = 1.0_f64 / t2595;
    let t2597 = t177 * t2596;
    let t2598 = t2492 * t745;
    let t2601 = t2514 * t745;
    let t2604 = t177 * t2491;
    let t2605 = t2492 * t2495;
    let t2608 = -0.70983522622222222221e-3_f64 * t123 * t2434 * t173 - 0.34246666666666666666e-1_f64 * t268 * t2531 * t731 - 2.0_f64 * t2537 * t2539 + 1.0_f64 * t724 * t2549 + 0.32163958997385070134e2_f64 * t2554 * t2557 + t2562 + t2522 + t2569 - t2579 - t2587 - 0.24415263074675393405e-3_f64 * t123 * t2434 * t186 - 0.10843581300301739842e-1_f64 * t268 * t2591 * t746 - 0.11696447245269292414e1_f64 * t2597 * t2598 + 0.5848223622634646207e0_f64 * t739 * t2601 + 0.17315859105681463759e2_f64 * t2604 * t2605;
    (t2596, t2608)
}
