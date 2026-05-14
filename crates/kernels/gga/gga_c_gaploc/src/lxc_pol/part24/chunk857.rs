//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 857/1270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk857<F: Float>(t7291: F, t883: F, t5641: F, t9805: F, t2365: F, t7292: F, t6111: F, t3295: F, t826: F, t825: F, t2021: F, t2672: F, t7372: F, t7634: F, t787: F) -> (F, F, F, F, F, F, F, F, F) {
    let t9806 = t883 * t7291;
    let t9807 = t5641 * t9806;
    let t9809 = 0.11502877786176224903e1 * t9805 * t9807;
    let t9810 = t2365 * t7292;
    let t9812 = 0.59584149919750711116e-1 * t6111 * t9810;
    let t9813 = t826 * t3295;
    let t9814 = t825 * t9813;
    let t9815 = 0.51123901271894332901e0 * t9814;
    let t9820 = t2021 * t2672;
    let t9822 = 0.29792074959875355558e-1 * t9820 * t7372;
    let t9823 = t787 * t7634;
    (t9807, t9809, t9810, t9812, t9813, t9815, t9820, t9822, t9823)
}
