//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 928/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk928<F: Float>(t31211: F, t500: F, t30494: F, t6317: F, t6316: F, t31165: F, t4231: F, t6368: F, t21321: F, t8244: F, t31146: F, t4230: F, t6322: F, t6321: F, t4204: F, t4203: F) -> (F, F, F, F, F, F, F) {
    let t31212 = t31211 * t500;
    let t31214 = t6317 * t30494;
    let t31215 = t6316 * t31214;
    let t31217 = t4231 * t31165;
    let t31218 = t6368 * t31217;
    let t31220 = t21321 * t8244;
    let t31222 = t4231 * t31146;
    let t31223 = t4230 * t31222;
    let t31225 = t6322 * t30494;
    let t31226 = t6321 * t31225;
    let t31228 = t4204 * t31146;
    let t31229 = t4203 * t31228;
    (t31212, t31215, t31218, t31220, t31223, t31226, t31229)
}
