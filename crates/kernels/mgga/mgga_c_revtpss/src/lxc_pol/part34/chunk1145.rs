//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1145/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1145<F: Float>(t113138: F, t113160: F, t113267: F, t113291: F, t113320: F, t113351: F, t113380: F, t113412: F, t892: F, t1468: F, t5962: F, t23421: F, t30: F, t23148: F, t1583: F, t25207: F) -> (F, F, F, F, F, F, F) {
    let t113415 = t113138 + t113160 + t113267 + t113291 + t113320 + t113351 + t113380 + t113412;
    let t113416 = t113415 * t892;
    let t113420 = t1468 * t5962;
    let t113424 = t30 * t23421;
    let t113428 = t30 * t23148;
    let t113432 = t5962 * t1583;
    let t113433 = t25207 * t113432;
    (t113415, t113416, t113420, t113424, t113428, t113432, t113433)
}
