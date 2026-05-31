//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 427/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk427<F: Float>(t601: F, t604: F, t1414: F, t162: F, t161: F, t1776: F) -> (F, F, F, F) {
    let t1778 = F::cast_from(288.0_f64) * t601 * t604;
    let t1779 = t162 * t1414;
    let t1780 = F::cast_from(1.0_f64) / t1779;
    let t1782 = F::cast_from(156.0_f64) * t161 * t1780;
    let t1783 = -t1776 + t1778 - t1782;
    (t1779, t1780, t1782, t1783)
}
