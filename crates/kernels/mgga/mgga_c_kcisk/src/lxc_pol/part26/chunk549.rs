//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 549/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk549<F: Float>(t1474: F, t4265: F, t140: F, t1477: F, t299: F, t3529: F, t41: F, t3532: F, t451: F, t1402: F, t442: F, t1390: F, t470: F, t1553: F, t1556: F, t1555: F, t547: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4266 = t4265 * t1474;
    let t4269 = t140 * t299 * t1477;
    let t4271 = t41 * t3529;
    let t4272 = t451 * t3532;
    let t4277 = t1402 * t442;
    let t4282 = t451 * t1390;
    let t4304 = 1.0 / t470;
    let t4324 = t1553 * t1556;
    let t4346 = 1.0 / t1555 / t547;
    (t4266, t4269, t4271, t4272, t4277, t4282, t4304, t4324, t4346)
}
