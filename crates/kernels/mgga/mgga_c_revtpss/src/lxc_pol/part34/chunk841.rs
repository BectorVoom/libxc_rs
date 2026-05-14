//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 841/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk841<F: Float>(t1261: F, t21192: F, t1010: F, t5843: F, t5378: F, t5381: F, t12884: F, t247: F, t6421: F, t1785: F, t5390: F, t5357: F, t5373: F, t140: F, t6658: F, t1222: F) -> (F, F, F, F, F, F, F) {
    let t21193 = t1261 * t21192;
    let t21213 = t5843 * t1010;
    let t21216 = t5381 * t5378;
    let t21233 = t247 * t12884 * t6421;
    let t21234 = t1261 * t21233;
    let t21242 = t1785 * t5390;
    let t21249 = t5373 * t5357;
    let t21251 = t140 * t6658;
    let t21252 = t1222 * t21251;
    (t21193, t21213, t21216, t21234, t21242, t21249, t21252)
}
