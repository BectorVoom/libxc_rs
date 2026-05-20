//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2694/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2694<F: Float>(t1353: F, t198: F, t3829: F, t13607: F, t13656: F, t1450: F, t39419: F, t39422: F, t46297: F, t46963: F, t47753: F, t47754: F, t47758: F, t47759: F, t47760: F, t47798: F, t47828: F, t47862: F, t47889: F, t47922: F, t48153: F, t48155: F, t48157: F, t48159: F, t48160: F, t48218: F, t49466: F, t49506: F, t49534: F, t532: F, t5536: F, t5591: F, t5627: F, t5783: F, t9547: F) -> F {
    let t49541 = t198 * t1353;
    let t49544 = t198 * t3829;
    let t49550 = -t47753 + t47754 + F::new(18.0) * t5536 * t9547 * t5627 - t47758 + t47759 + t47760 - t46297 - t39419 - t39422 + t198 * t532 * (t47798 + t47828 + t47862 + t47889 + t47922 + t49466 + t49506 + t49534) * t1450 + F::new(18.0) * t49541 * t13607 - t48153 - t48155 + F::new(18.0) * t49544 * t5783 + t48157 + t48159 - t48160 + t48218 + F::new(18.0) * t198 * t13656 * t5591 - t46963;
    t49550
}
