//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 859/1331 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk859<F: Float>(t10264: F, t827: F, t6188: F, t800: F, t1087: F, t2415: F, t6172: F, t3238: F, t2448: F, t3197: F, t799: F, t3250: F, t828: F) -> (F, F, F, F) {
    let t10265 = t10264 * t827;
    let t10266 = t800 * t6188;
    let t10267 = t10265 * t10266;
    let t10269 = t2415 * t1087;
    let t10270 = t10269 * t6172;
    let t10271 = t3238 * t10270;
    let t10273 = t3197 * t2448;
    let t10274 = t799 * t10273;
    let t10276 = t828 * t3250;
    (t10267, t10271, t10274, t10276)
}
