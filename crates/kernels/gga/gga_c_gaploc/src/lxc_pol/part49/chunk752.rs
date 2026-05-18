//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 752/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk752<F: Float>(t12806: F, t6320: F, t2268: F, t3148: F, t988: F, t10122: F, t883: F, t2325: F, t882: F, t12353: F, t12361: F, t12367: F) -> (F, F, F, F, F, F, F, F, F) {
    let t12807 = t6320 * t12806;
    let t12809 = F::new(0.17073003981405689759e0) * t2268 * t12807;
    let t12810 = t3148 * t988;
    let t12812 = F::new(0.28455006635676149599e-1) * t2268 * t12810;
    let t12819 = t883 * t10122;
    let t12820 = t2325 * t12819;
    let t12821 = t882 * t12820;
    let t12823 = F::new(0.23712505529730124666e-2) * t12353;
    let t12824 = F::new(0.23712505529730124666e-2) * t12361;
    let t12825 = F::new(0.23712505529730124666e-2) * t12367;
    (t12807, t12809, t12810, t12812, t12820, t12821, t12823, t12824, t12825)
}
