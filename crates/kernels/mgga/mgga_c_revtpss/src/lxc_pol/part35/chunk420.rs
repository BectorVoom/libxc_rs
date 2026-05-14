//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 420/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk420<F: Float>(t675: F, t738: F, t182: F, t737: F, t177: F, t2492: F, t745: F, t2514: F, t2491: F, t2495: F, t123: F, t173: F, t186: F, t2434: F, t2522: F, t2531: F, t2537: F, t2539: F, t2549: F, t2554: F, t2557: F, t2562: F, t2569: F, t2579: F, t2587: F, t268: F, t724: F, t731: F, t739: F, t746: F) -> (F, F, F, F, F, F, F, F) {
    let t2591 = t675 * t738;
    let t2595 = t737 * t182;
    let t2596 = 1.0 / t2595;
    let t2597 = t177 * t2596;
    let t2598 = t2492 * t745;
    let t2601 = t2514 * t745;
    let t2604 = t177 * t2491;
    let t2605 = t2492 * t2495;
    let t2608 = -0.70983522622222222221e-3 * t123 * t2434 * t173 - 0.34246666666666666666e-1 * t268 * t2531 * t731 - 2.0 * t2537 * t2539 + 1.0 * t724 * t2549 + 0.32163958997385070134e2 * t2554 * t2557 + t2562 + t2522 + t2569 - t2579 - t2587 - 0.24415263074675393405e-3 * t123 * t2434 * t186 - 0.10843581300301739842e-1 * t268 * t2591 * t746 - 0.11696447245269292414e1 * t2597 * t2598 + 0.5848223622634646207e0 * t739 * t2601 + 0.17315859105681463759e2 * t2604 * t2605;
    (t2591, t2596, t2597, t2598, t2601, t2604, t2605, t2608)
}
