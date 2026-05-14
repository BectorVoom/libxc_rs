//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 621/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk621<F: Float>(t10122: F, t883: F, t2325: F, t882: F, t12353: F, t12361: F, t12367: F, t3133: F, t7937: F, t2268: F, t12425: F, t10166: F, t3129: F, t9074: F, t12428: F, t3152: F, t988: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t12819 = t883 * t10122;
    let t12820 = t2325 * t12819;
    let t12821 = t882 * t12820;
    let t12823 = 0.23712505529730124666e-2 * t12353;
    let t12824 = 0.23712505529730124666e-2 * t12361;
    let t12825 = 0.23712505529730124666e-2 * t12367;
    let t12826 = t7937 * t3133;
    let t12828 = 0.34146007962811379518e0 * t2268 * t12826;
    let t12829 = 0.47425011059460249332e-2 * t12425;
    let t12830 = t10166 * t3129;
    let t12831 = t9074 * t12830;
    let t12832 = 0.71137516589190373998e-2 * t12831;
    let t12833 = 0.71137516589190373998e-2 * t12428;
    let t12834 = t3152 * t988;
    (t12820, t12821, t12823, t12824, t12825, t12826, t12828, t12829, t12830, t12832, t12833, t12834)
}
