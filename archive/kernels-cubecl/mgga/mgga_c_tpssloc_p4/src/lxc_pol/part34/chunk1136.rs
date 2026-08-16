//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1136/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1136<F: Float>(t28182: F, t6914: F, t22893: F, t28142: F, t80681: F, t28143: F, t80727: F, t28160: F, t6883: F, t6396: F, t80820: F, t28101: F, t80958: F) -> (F, F, F, F, F, F) {
    let t97148 = t6914 * t28182;
    let t97161 = t80681 * t22893 * t28142;
    let t97179 = t80727 * t28143;
    let t97200 = t6883 * t28160;
    let t97219 = t80820 * t6396;
    let t97238 = t80958 * t28101;
    (t97148, t97161, t97179, t97200, t97219, t97238)
}
