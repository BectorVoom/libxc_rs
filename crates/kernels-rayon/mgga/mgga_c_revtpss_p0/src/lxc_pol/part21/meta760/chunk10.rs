//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2694/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2694(t1353: f64, t198: f64, t3829: f64, t13607: f64, t13656: f64, t1450: f64, t39419: f64, t39422: f64, t46297: f64, t46963: f64, t47753: f64, t47754: f64, t47758: f64, t47759: f64, t47760: f64, t47798: f64, t47828: f64, t47862: f64, t47889: f64, t47922: f64, t48153: f64, t48155: f64, t48157: f64, t48159: f64, t48160: f64, t48218: f64, t49466: f64, t49506: f64, t49534: f64, t532: f64, t5536: f64, t5591: f64, t5627: f64, t5783: f64, t9547: f64) -> f64 {
    let t49541 = t198 * t1353;
    let t49544 = t198 * t3829;
    let t49550 = -t47753 + t47754 + 18.0_f64 * t5536 * t9547 * t5627 - t47758 + t47759 + t47760 - t46297 - t39419 - t39422 + t198 * t532 * (t47798 + t47828 + t47862 + t47889 + t47922 + t49466 + t49506 + t49534) * t1450 + 18.0_f64 * t49541 * t13607 - t48153 - t48155 + 18.0_f64 * t49544 * t5783 + t48157 + t48159 - t48160 + t48218 + 18.0_f64 * t198 * t13656 * t5591 - t46963;
    t49550
}
