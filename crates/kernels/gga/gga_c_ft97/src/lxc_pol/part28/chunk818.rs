//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 818/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk818<F: Float>(t1642: F, t2112: F, t378: F, t9236: F, t1378: F, t9438: F, t2101: F, t5929: F, t582: F, t5935: F, t5842: F, t604: F, t23571: F, t50235: F, t5617: F, t984: F) -> (F, F, F, F, F, F, F, F, F) {
    let t95340 = t1642 * t2112;
    let t95344 = t378 * t9236;
    let t95403 = t1378 * t9438;
    let t95696 = t2101 * t5929;
    let t95767 = t582 * t5929;
    let t95789 = t2101 * t5935;
    let t95813 = t604 * t5842;
    let t95842 = t50235 * t23571;
    let t100089 = t5617 * t984;
    (t95340, t95344, t95403, t95696, t95767, t95789, t95813, t95842, t100089)
}
