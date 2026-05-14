//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 723/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk723<F: Float>(t11280: F, t20883: F, t6525: F, t42539: F, t42546: F, t10166: F, t10252: F, t9074: F, t13296: F, t203: F, t550: F, t36274: F, t4261: F, t35913: F, t19532: F, t35959: F) -> (F, F, F, F, F, F, F, F, F) {
    let t44261 = t6525 * t11280 * t20883;
    let t44262 = 0.35568758294595186999e-2 * t44261;
    let t44263 = 0.47425011059460249332e-2 * t42539;
    let t44264 = 0.94850022118920498664e-2 * t42546;
    let t44266 = t9074 * t10166 * t10252;
    let t44267 = 0.71137516589190373998e-2 * t44266;
    let t44268 = t203 * t13296;
    let t44269 = t550 * t44268;
    let t44277 = t6525 * t4261 * t36274;
    let t44278 = 0.23712505529730124666e-2 * t44277;
    let t44280 = t9074 * t4261 * t35913;
    let t44281 = 0.47425011059460249332e-2 * t44280;
    let t44283 = t9074 * t19532 * t35959;
    (t44262, t44263, t44264, t44267, t44268, t44269, t44278, t44281, t44283)
}
