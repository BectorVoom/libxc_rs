//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 809/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk809<F: Float>(t42546: F, t10166: F, t10252: F, t9074: F, t36274: F, t4261: F, t6525: F, t35913: F, t19532: F, t35959: F, t123: F, t37975: F) -> (F, F, F, F, F, F) {
    let t44264 = F::cast_from(0.94850022118920498664e-2_f64) * t42546;
    let t44266 = t9074 * t10166 * t10252;
    let t44267 = F::cast_from(0.71137516589190373998e-2_f64) * t44266;
    let t44277 = t6525 * t4261 * t36274;
    let t44278 = F::cast_from(0.23712505529730124666e-2_f64) * t44277;
    let t44280 = t9074 * t4261 * t35913;
    let t44281 = F::cast_from(0.47425011059460249332e-2_f64) * t44280;
    let t44283 = t9074 * t19532 * t35959;
    let t44284 = F::cast_from(0.71137516589190373998e-2_f64) * t44283;
    let t44285 = t37975 * t123;
    (t44264, t44267, t44278, t44281, t44284, t44285)
}
