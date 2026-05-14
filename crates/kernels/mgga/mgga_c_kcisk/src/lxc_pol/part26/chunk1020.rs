//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1020/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1020<F: Float>(t26796: F, t4231: F, t4230: F, t1440: F, t8077: F, t4204: F, t6331: F, t14592: F, t8271: F, t21290: F, t2263: F, t6360: F, t6388: F, t25296: F, t25350: F, t492: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t27188 = t4231 * t26796;
    let t27189 = t4230 * t27188;
    let t27191 = t8077 * t1440;
    let t27192 = t4204 * t27191;
    let t27193 = t6331 * t27192;
    let t27195 = t14592 * t8271;
    let t27197 = t21290 * t2263;
    let t27199 = t6388 * t6360;
    let t27201 = t4204 * t25296;
    let t27202 = t6331 * t27201;
    let t27204 = t25350 * t492;
    (t27188, t27189, t27191, t27193, t27195, t27197, t27199, t27201, t27202, t27204)
}
