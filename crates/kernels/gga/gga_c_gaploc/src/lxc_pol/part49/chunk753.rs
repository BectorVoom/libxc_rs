//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 753/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk753<F: Float>(t3133: F, t7937: F, t2268: F, t12425: F, t10166: F, t3129: F, t9074: F, t12428: F, t3152: F, t988: F, t3340: F, t894: F) -> (F, F, F, F, F, F, F, F, F) {
    let t12826 = t7937 * t3133;
    let t12828 = F::new(0.34146007962811379518e0) * t2268 * t12826;
    let t12829 = F::new(0.47425011059460249332e-2) * t12425;
    let t12830 = t10166 * t3129;
    let t12831 = t9074 * t12830;
    let t12832 = F::new(0.71137516589190373998e-2) * t12831;
    let t12833 = F::new(0.71137516589190373998e-2) * t12428;
    let t12834 = t3152 * t988;
    let t12836 = F::new(0.28455006635676149599e-1) * t2268 * t12834;
    let t12837 = t894 * t3340;
    (t12826, t12828, t12829, t12830, t12832, t12833, t12834, t12836, t12837)
}
