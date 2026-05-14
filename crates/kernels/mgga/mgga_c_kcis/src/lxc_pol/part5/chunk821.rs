//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 821/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk821<F: Float>(t1471: F, t1472: F, t6284: F, t3780: F, t6957: F, t542: F, t1961: F, t5463: F, t3786: F, t1477: F, t6964: F, t3814: F, t7122: F, t1482: F, t7141: F, t1924: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t7230 = t1471 * t1472 * t6284;
    let t7233 = t3780 * t6957;
    let t7234 = t542 * t7233;
    let t7237 = t5463 * t1961;
    let t7238 = t3786 * t7237;
    let t7241 = t1477 * t6964;
    let t7242 = t542 * t7241;
    let t7245 = t3814 * t7122;
    let t7246 = t542 * t7245;
    let t7249 = t1482 * t7141;
    let t7250 = t542 * t7249;
    let t7253 = t1924 * t1924;
    (t7230, t7233, t7234, t7237, t7238, t7241, t7242, t7245, t7246, t7249, t7250, t7253)
}
