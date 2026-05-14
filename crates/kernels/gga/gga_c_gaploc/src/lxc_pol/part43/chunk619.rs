//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 619/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk619<F: Float>(t12766: F, t2343: F, t2268: F, t12383: F, t12386: F, t12392: F, t12395: F, t12397: F, t12400: F, t471: F, t3334: F, t871: F, t12412: F, t10318: F, t2321: F, t9074: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t12767 = t2343 * t12766;
    let t12769 = 0.56910013271352299198e-1 * t2268 * t12767;
    let t12782 = -3.0 / 256.0 * t12383 - 27.0 / 8192.0 * t12386 + 27.0 / 524288.0 * t12392 - 9.0 / 524288.0 * t12395 + 9.0 / 8192.0 * t12397 + t12400 / 256.0;
    let t12783 = t12782 * t471;
    let t12784 = t3334 * t871;
    let t12787 = 9.0 / 256.0 * t12383;
    let t12788 = 9.0 / 8192.0 * t12386;
    let t12789 = 3.0 / 8192.0 * t12397;
    let t12790 = 3.0 / 256.0 * t12400;
    let t12791 = 2.0 * t12412;
    let t12797 = t10318 * t2321;
    let t12798 = t9074 * t12797;
    (t12767, t12769, t12782, t12783, t12784, t12787, t12788, t12789, t12790, t12791, t12797, t12798)
}
