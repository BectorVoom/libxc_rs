//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 689/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk689<F: Float>(t12806: F, t6320: F, t2268: F, t3148: F, t988: F, t12353: F, t12361: F, t12367: F, t3133: F, t7937: F, t12425: F, t10166: F, t3129: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t12807 = t6320 * t12806;
    let t12809 = F::cast_from(0.17073003981405689759e0_f64) * t2268 * t12807;
    let t12810 = t3148 * t988;
    let t12812 = F::cast_from(0.28455006635676149599e-1_f64) * t2268 * t12810;
    let t12823 = F::cast_from(0.23712505529730124666e-2_f64) * t12353;
    let t12824 = F::cast_from(0.23712505529730124666e-2_f64) * t12361;
    let t12825 = F::cast_from(0.23712505529730124666e-2_f64) * t12367;
    let t12826 = t7937 * t3133;
    let t12828 = F::cast_from(0.34146007962811379518e0_f64) * t2268 * t12826;
    let t12829 = F::cast_from(0.47425011059460249332e-2_f64) * t12425;
    let t12830 = t10166 * t3129;
    (t12807, t12809, t12810, t12812, t12823, t12824, t12825, t12826, t12828, t12829, t12830)
}
