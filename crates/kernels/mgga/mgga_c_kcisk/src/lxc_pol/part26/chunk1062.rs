//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1062/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1062<F: Float>(t27046: F, t27048: F, t27050: F, t27052: F, t27054: F, t27056: F, t27059: F, t27062: F, t27063: F, t27066: F, t27432: F, t240: F, t28138: F, t15772: F, t7706: F, t3277: F, t7710: F) -> (F, F, F) {
    let t28139 = -t27046 + t27048 + t27050 - t27052 + t27054 - t27056 + t27059 - t27062 + t27063 - t27066 + t27432;
    let t28142 = t27046 - t27048 - t27050 + t27052 - t27054 + t27056 - t27059 + t27062 - t27063 + t27066 - t27432 + t240 * (t28138 + t28139);
    let t28153 = t15772 * t7706;
    let t28158 = t3277 * t7710;
    (t28142, t28153, t28158)
}
