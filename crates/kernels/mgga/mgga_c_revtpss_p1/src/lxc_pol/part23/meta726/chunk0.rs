//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2492/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2492<F: Float>(t49186: F, t10142: F, t14113: F, t49180: F, t10136: F, t14239: F, t4101: F, t5740: F, t9288: F, t40270: F, t5737: F, t10073: F, t14207: F) -> (F, F, F, F, F, F) {
    let t49187 = F::cast_from(0.69394917116090352834e-2_f64) * t49186;
    let t49189 = t49180 * t14113 * t10142;
    let t49190 = F::cast_from(0.34697458558045176417e-2_f64) * t49189;
    let t49198 = t14239 * t10136;
    let t49199 = F::cast_from(0.39029762157531132076e-1_f64) * t49198;
    let t49203 = t4101 * t5740 * t9288;
    let t49210 = t40270 * t5737;
    let t49238 = t10073 * t14207;
    (t49187, t49190, t49199, t49203, t49210, t49238)
}
