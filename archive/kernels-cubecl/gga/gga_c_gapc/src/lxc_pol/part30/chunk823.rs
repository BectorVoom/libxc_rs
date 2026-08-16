//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 823/1331 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk823<F: Float>(t2660: F, t8624: F, t7330: F, t7335: F, t9810: F, t1077: F, t2713: F, t3307: F, t910: F, t1069: F, t2508: F, t191: F, t2674: F) -> (F, F, F, F, F, F) {
    let t9815 = t2660 * t8624;
    let t9816 = t9815 * t7330;
    let t9818 = t9810 * t7335;
    let t9820 = t1077 * t2713;
    let t9822 = t3307 * t910;
    let t9824 = t1069 * t2508;
    let t9826 = t2674 * t191;
    (t9816, t9818, t9820, t9822, t9824, t9826)
}
