//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 604/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk604<F: Float>(t3742: F, t4231: F, t4230: F, t3784: F, t492: F, t3786: F, t499: F, t498: F, t1284: F, t3777: F, t487: F, t486: F, t196: F, t3729: F, t306: F, t476: F) -> (F, F, F, F, F, F, F, F) {
    let t4232 = t4231 * t3742;
    let t4233 = t4230 * t4232;
    let t4235 = t3784 * t492;
    let t4236 = t499 * t3786;
    let t4237 = t498 * t4236;
    let t4238 = t4235 * t4237;
    let t4240 = t1284 * t3777;
    let t4241 = t487 * t4240;
    let t4242 = t486 * t4241;
    let t4244 = t3729 * t196;
    let t4253 = t476 * t306;
    (t4232, t4233, t4237, t4238, t4241, t4242, t4244, t4253)
}
