//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 687/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk687<F: Float>(t12792: F, t493: F, t492: F, t10318: F, t2321: F, t9074: F, t3158: F, t993: F, t2268: F, t10268: F, t4261: F, t2854: F, t3085: F) -> (F, F, F, F, F, F, F, F, F) {
    let t12793 = t493 * t12792;
    let t12794 = t492 * t12793;
    let t12797 = t10318 * t2321;
    let t12798 = t9074 * t12797;
    let t12799 = F::new(0.23712505529730124666e-2) * t12798;
    let t12800 = t3158 * t993;
    let t12802 = F::new(0.19918504644973304719e0) * t2268 * t12800;
    let t12803 = t4261 * t10268;
    let t12804 = t9074 * t12803;
    let t12805 = F::new(0.47425011059460249332e-2) * t12804;
    let t12806 = t2854 * t3085;
    (t12793, t12794, t12797, t12799, t12800, t12802, t12803, t12805, t12806)
}
