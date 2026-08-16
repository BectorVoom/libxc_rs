//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2617/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2617(t162: f64, t48187: f64, t48214: f64, t189: f64, t512: f64, t46967: f64, t39419: f64, t39422: f64, t46297: f64, t46963: f64, t47753: f64, t47754: f64, t47758: f64, t47759: f64, t47760: f64, t48153: f64, t48155: f64, t48157: f64, t48159: f64, t48160: f64) -> (f64, f64, f64, f64) {
    let t48216 = (t48187 + t48214) * t162;
    let t48218 = t512 * t48216 * t189;
    let t48219 = 60.0_f64 * t46967;
    let t48220 = -t47753 + t47754 - t47758 + t47759 + t47760 - t46297 - t39419 - t39422 - t48153 - t48155 + t48157 + t48159 - t48160 + t48218 - t46963 + t48219;
    (t48216, t48218, t48219, t48220)
}
