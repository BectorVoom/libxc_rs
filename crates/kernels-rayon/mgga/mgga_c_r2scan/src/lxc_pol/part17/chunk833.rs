//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 833/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk833(t2562: f64, t2625: f64, t360: f64, t3016: f64, t537: f64, t2124: f64, t495: f64, t2122: f64, t2133: f64, t2139: f64, t2564: f64, t2575: f64, t2579: f64, t2582: f64, t2584: f64, t5108: f64, t6106: f64, t6583: f64, t7362: f64, t7367: f64, t7377: f64, t7461: f64, t7512: f64, t7984: f64, t7987: f64, t8770: f64, t8775: f64, t8780: f64, t8785: f64, t8792: f64, t8796: f64, t8800: f64, t8804: f64) -> (f64, f64, f64) {
    let t8807 = t2562 * t2625;
    let t8808 = t360 * t8807;
    let t8811 = t537 * t3016;
    let t8813 = t2124 * t8811 * t495;
    let t8818 = -0.2600466522016280569e0_f64 * t5108 * t8770 - t7362 - t7367 - t7377 - 0.86682217400542685632e-1_f64 * t6583 * t8775 - 0.43341108700271342816e-1_f64 * t2582 * t8780 - 0.5200933044032561138e0_f64 * t6106 * t8785 + 0.86682217400542685632e-1_f64 * t7984 * t2575 + 0.2600466522016280569e0_f64 * t7987 * t2579 - 0.86682217400542685632e-1_f64 * t8792 * t2584 + 0.43341108700271342816e-1_f64 * t2133 * t8796 + 0.13002332610081402845e0_f64 * t2139 * t8800 - 0.5200933044032561138e0_f64 * t7512 * t8804 - 0.10401866088065122276e1_f64 * t7461 * t8808 + 0.54878743191129263322e-1_f64 * t2122 * t8813 + 0.86682217400542685632e-1_f64 * t7984 * t2564;
    (t8807, t8813, t8818)
}
