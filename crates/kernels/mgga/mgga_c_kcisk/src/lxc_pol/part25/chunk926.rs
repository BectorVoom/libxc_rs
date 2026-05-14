//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 926/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk926<F: Float>(t22: F, t5815: F, t6831: F, t10663: F, t2372: F, t4624: F, t6828: F, t827: F, t6825: F, t16013: F, t4726: F, t26: F, t10621: F, t15999: F, t16009: F, t5744: F) -> (F, F, F, F, F, F, F, F) {
    let t16391 = t22 * t5815;
    let t16392 = t16391 * t6831;
    let t16395 = t10663 * t2372;
    let t16396 = t16395 * t4624;
    let t16398 = t827 * t6828;
    let t16399 = 0.21908444444444444444e0 * t16398;
    let t16400 = t827 * t6825;
    let t16402 = t4726 * t16013;
    let t16403 = t26 * t16402;
    let t16405 = t10621 * t15999;
    let t16406 = t26 * t16405;
    let t16409 = t4726 * t16009;
    let t16410 = t5744 * t16409;
    (t16392, t16396, t16398, t16399, t16400, t16403, t16406, t16410)
}
