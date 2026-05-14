//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 814/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk814<F: Float>(t10256: F, t10257: F, t10243: F, t276: F, t6194: F, t10246: F, t268: F, t2902: F, t827: F, t6188: F, t800: F, t1087: F, t2415: F, t6172: F, t3238: F, t2448: F, t3197: F) -> (F, F, F, F, F, F) {
    let t10258 = t10256 * t10257;
    let t10260 = t10243 * t276;
    let t10261 = t10260 * t6194;
    let t10262 = t10261 * t10246;
    let t10264 = t2902 * t268;
    let t10265 = t10264 * t827;
    let t10266 = t800 * t6188;
    let t10267 = t10265 * t10266;
    let t10269 = t2415 * t1087;
    let t10270 = t10269 * t6172;
    let t10271 = t3238 * t10270;
    let t10273 = t3197 * t2448;
    (t10258, t10262, t10264, t10267, t10271, t10273)
}
