//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 849/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk849<F: Float>(t2562: F, t38974: F, t883: F, t943: F, t13765: F, t4349: F, t605: F, t13838: F, t5552: F, t3718: F, t6553: F, t12148: F, t2355: F, t1339: F, t1537: F, t46849: F, t590: F) -> (F, F, F, F, F, F) {
    let t47772 = t943 * t2562 * t883 * t38974;
    let t47784 = t4349 * t13765 * t605;
    let t47786 = t5552 * t13838;
    let t47790 = t6553 * t3718;
    let t47791 = t2355 * t12148;
    let t47794 = t1537 * t1339 * t46849 * t590;
    (t47772, t47784, t47786, t47790, t47791, t47794)
}
