//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 808/1331 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk808<F: Float>(t2255: F, t2982: F, t9623: F, t2619: F, t9128: F, t3388: F, t916: F, t3392: F, t2405: F, t3391: F, t197: F, t7975: F) -> (F, F, F, F, F, F, F) {
    let t9624 = t2982 * t2255;
    let t9625 = t9623 * t9624;
    let t9627 = t2619 * t9128;
    let t9628 = t9627 * t3388;
    let t9630 = t916 * t9128;
    let t9631 = t9630 * t3392;
    let t9635 = t2982 * t2405;
    let t9636 = t3391 * t9635;
    let t9638 = t197 * t7975;
    (t9624, t9625, t9628, t9631, t9635, t9636, t9638)
}
