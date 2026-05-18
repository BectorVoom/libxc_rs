//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 672/1429 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk672<F: Float>(t1698: F, t442: F, t619: F, t457: F, t681: F, t1903: F, t1908: F, t198: F, t137: F, t567: F) -> (F, F, F, F, F, F) {
    let t5189 = t1698 * t442;
    let t5190 = t619 * t5189;
    let t5199 = t681 * t457;
    let t5211 = t1903 * M_PI;
    let t5214 = t198 * t1908;
    let t5215 = t5214 * t681;
    let t5216 = t567 * t137;
    (t5190, t5199, t5211, t5214, t5215, t5216)
}
