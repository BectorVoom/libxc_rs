//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 617/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk617<F: Float>(t13261: F, t2343: F, t2268: F, t3565: F, t6470: F, t882: F, t11264: F, t2492: F, t11172: F, t6485: F, t883: F, t11259: F, t874: F) -> (F, F, F, F, F, F, F, F, F) {
    let t13262 = t2343 * t13261;
    let t13264 = 0.56910013271352299198e-1 * t2268 * t13262;
    let t13265 = t3565 * t6470;
    let t13266 = t882 * t13265;
    let t13267 = 0.35568758294595186999e-2 * t13266;
    let t13268 = t11264 * t2492;
    let t13270 = 0.34146007962811379518e0 * t2268 * t13268;
    let t13273 = t6485 * t883 * t11172;
    let t13274 = t882 * t13273;
    let t13275 = 0.23712505529730124666e-2 * t13274;
    let t13276 = t11259 * t874;
    (t13262, t13264, t13265, t13267, t13268, t13270, t13273, t13275, t13276)
}
