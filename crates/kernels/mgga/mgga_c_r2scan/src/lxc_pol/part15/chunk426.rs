//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 426/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk426<F: Float>(t601: F, t604: F, t1414: F, t162: F, t161: F, t1776: F) -> (F, F, F, F) {
    let t1778 = F::new(288.0) * t601 * t604;
    let t1779 = t162 * t1414;
    let t1780 = F::new(1.0) / t1779;
    let t1782 = F::new(156.0) * t161 * t1780;
    let t1783 = -t1776 + t1778 - t1782;
    (t1779, t1780, t1782, t1783)
}
