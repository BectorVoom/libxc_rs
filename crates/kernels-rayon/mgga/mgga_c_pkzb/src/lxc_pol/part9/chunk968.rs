//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 968/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk968(t2942: f64, t7606: f64, t2883: f64, t735: f64, t2887: f64, t2891: f64, t2948: f64, t5591: f64, t5595: f64, t5597: f64, t5607: f64, t5609: f64, t5614: f64, t5620: f64, t5630: f64, t757: f64, t7578: f64, t7582: f64, t7585: f64, t7586: f64, t7591: f64, t7594: f64, t7598: f64, t7602: f64) -> (f64, f64) {
    let t7607 = t2942 * t7606;
    let t7617 = t735 * t2883 / 54.0_f64;
    let t7618 = 0.21437009059034868486e-3_f64 * t757 * t7578 - 0.47637797908966374413e-4_f64 * t7582 - t7585 - t7586 * t2891 / 9.0_f64 + t7591 + t2887 * t7594 / 24.0_f64 + t2887 * t7598 / 48.0_f64 - t2887 * t7602 / 16.0_f64 - 0.13719685797782315831e-1_f64 * t7607 * t2948 + t5591 + 11.0_f64 / 324.0_f64 * t5595 + t5597 / 81.0_f64 + 0.48272968547752592739e-2_f64 * t5607 + 0.5081365110289746604e-3_f64 * t5609 + t5614 + 0.30488190661738479624e-2_f64 * t5620 + 0.85748036236139473944e-3_f64 * t5630 + t7617;
    (t7607, t7618)
}
