//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2939/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2939(t10189: f64, t5842: f64, t2986: f64, t2990: f64, t13847: f64, t13861: f64, t17841: f64, t2987: f64, t10186: f64, t13812: f64, t17788: f64, t17805: f64, t17811: f64, t17814: f64, t17818: f64, t17854: f64, t17867: f64, t4531: f64, t47966: f64, t48184: f64) -> f64 {
    let t61189 = t10189 * t5842;
    let t61191 = t2986 * t61189 * t2990;
    let t61200 = t2986 * t13847 * t13861;
    let t61210 = t2987 * t17841;
    let t61214 = 0.22222222222222222222e-2_f64 * t2986 * t4531 * t47966 - 0.33333333333333333332e-2_f64 * t2986 * t48184 * t13812 - 0.18518518518518518518e-3_f64 * t61191 + 0.14814814814814814814e-2_f64 * t10186 * t17805 + 0.29629629629629629628e-2_f64 * t10186 * t17867 + 0.29629629629629629628e-2_f64 * t10186 * t17788 - 0.37037037037037037036e-3_f64 * t61200 - 0.88888888888888888885e-2_f64 * t10186 * t17811 + 0.59259259259259259257e-2_f64 * t10186 * t17814 + 0.11851851851851851851e-1_f64 * t10186 * t17854 - 0.29629629629629629628e-2_f64 * t10186 * t17818 - 0.55555555555555555554e-3_f64 * t2986 * t61210 * t2990;
    t61214
}
