//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2617/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2617<F: Float>(t162: F, t48187: F, t48214: F, t189: F, t512: F, t46967: F, t39419: F, t39422: F, t46297: F, t46963: F, t47753: F, t47754: F, t47758: F, t47759: F, t47760: F, t48153: F, t48155: F, t48157: F, t48159: F, t48160: F) -> (F, F, F, F) {
    let t48216 = (t48187 + t48214) * t162;
    let t48218 = t512 * t48216 * t189;
    let t48219 = F::cast_from(60.0_f64) * t46967;
    let t48220 = -t47753 + t47754 - t47758 + t47759 + t47760 - t46297 - t39419 - t39422 - t48153 - t48155 + t48157 + t48159 - t48160 + t48218 - t46963 + t48219;
    (t48216, t48218, t48219, t48220)
}
