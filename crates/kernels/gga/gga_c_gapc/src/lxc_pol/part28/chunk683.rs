//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 683/1429 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk683<F: Float>(t103: F, t1906: F, t203: F, t5698: F, t5700: F, t169: F, t4048: F, t4054: F, t442: F, t505: F, t682: F, t3141: F) -> (F, F, F, F) {
    let t5703 = t1906 * t5698 * t203 * t5700 * t103;
    let t5708 = t169 * t4048;
    let t5713 = t4054 * t442;
    let t5721 = t682 * t505;
    let t5722 = t3141 * t5721;
    (t5703, t5708, t5713, t5722)
}
