//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 929/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk929<F: Float>(t2573: F, t8778: F, t360: F, t2551: F, t2562: F, t2654: F, t2625: F, t3016: F, t537: F, t2124: F, t495: F, t2122: F, t2133: F, t2139: F, t2564: F, t2575: F, t2579: F, t2582: F, t2584: F, t5108: F, t6106: F, t6583: F, t7362: F, t7367: F, t7377: F, t7461: F, t7512: F, t7984: F, t7987: F, t8770: F, t8775: F, t8780: F, t8785: F, t8792: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t8795 = t8778 * t2573;
    let t8796 = t360 * t8795;
    let t8799 = t8778 * t2551;
    let t8800 = t360 * t8799;
    let t8803 = t2562 * t2654;
    let t8804 = t360 * t8803;
    let t8807 = t2562 * t2625;
    let t8808 = t360 * t8807;
    let t8811 = t537 * t3016;
    let t8813 = t2124 * t8811 * t495;
    let t8818 = -0.2600466522016280569e0 * t5108 * t8770 - t7362 - t7367 - t7377 - 0.86682217400542685632e-1 * t6583 * t8775 - 0.43341108700271342816e-1 * t2582 * t8780 - 0.5200933044032561138e0 * t6106 * t8785 + 0.86682217400542685632e-1 * t7984 * t2575 + 0.2600466522016280569e0 * t7987 * t2579 - 0.86682217400542685632e-1 * t8792 * t2584 + 0.43341108700271342816e-1 * t2133 * t8796 + 0.13002332610081402845e0 * t2139 * t8800 - 0.5200933044032561138e0 * t7512 * t8804 - 0.10401866088065122276e1 * t7461 * t8808 + 0.54878743191129263322e-1 * t2122 * t8813 + 0.86682217400542685632e-1 * t7984 * t2564;
    (t8795, t8796, t8799, t8800, t8803, t8804, t8807, t8808, t8811, t8813, t8818)
}
