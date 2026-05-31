//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2550/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2550<F: Float>(t54122: F, t1011: F, t3252: F, t4574: F, t697: F, t11263: F, t4879: F, t43537: F, t53668: F, t11817: F, t4858: F, t1045: F, t606: F) -> (F, F, F, F, F, F) {
    let t54123 = t54122 / F::cast_from(216.0_f64);
    let t54126 = t1011 * t697 * t3252 * t4574;
    let t54127 = t54126 / F::cast_from(324.0_f64);
    let t54147 = t4879 * t11263;
    let t54148 = F::cast_from(0.14291339372689912324e-3_f64) * t54147;
    let t54316 = t43537 * t53668;
    let t54387 = t4858 * t11817;
    let t54388 = F::cast_from(0.14291339372689912324e-3_f64) * t54387;
    let t54397 = t1045 * t606;
    (t54123, t54127, t54148, t54316, t54388, t54397)
}
