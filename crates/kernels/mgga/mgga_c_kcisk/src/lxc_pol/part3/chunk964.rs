//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 964/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk964<F: Float>(t1216: F, t3929: F, t1163: F, t13401: F, t1340: F, t1339: F, t1447: F, t3805: F, t1406: F, t3915: F, t415: F, t3532: F, t382: F) -> (F, F, F, F, F) {
    let t14242 = t1216 * t3929;
    let t14245 = t13401 * t1163;
    let t14246 = t1340 * t14245;
    let t14247 = t1339 * t14246;
    let t14250 = t3805 * t1447;
    let t14252 = t1406 * t3915;
    let t14253 = t415 * t14252;
    let t14255 = t382 * t3532;
    (t14242, t14247, t14250, t14253, t14255)
}
