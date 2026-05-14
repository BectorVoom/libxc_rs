//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 625/1125 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk625<F: Float>(t103: F, t1906: F, t203: F, t5698: F, t5700: F, t169: F, t4048: F, t4054: F, t442: F, t505: F, t682: F, t3141: F, t457: F, t662: F, t481: F, t674: F) -> (F, F, F, F, F, F, F) {
    let t5703 = t1906 * t5698 * t203 * t5700 * t103;
    let t5708 = t169 * t4048;
    let t5713 = t4054 * t442;
    let t5721 = t682 * t505;
    let t5722 = t3141 * t5721;
    let t5726 = t662 * t457;
    let t5727 = t3141 * t5726;
    let t5730 = t481 * t505;
    let t5741 = t1906 * t674;
    (t5703, t5708, t5713, t5722, t5727, t5730, t5741)
}
