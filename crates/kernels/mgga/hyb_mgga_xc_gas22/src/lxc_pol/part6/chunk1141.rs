//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1141/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1141<F: Float>(t1815: F, t19: F, t3114: F, t3118: F, t7884: F, t8169: F, t1819: F, t555: F, t7909: F, t7898: F, t8185: F, t20685: F, t24: F, t2988: F, t557: F, t13: F, t20075: F, t2969: F) -> (F, F, F, F, F, F, F, F) {
    let t23572 = t19 * t1815 * t3114;
    let t23575 = t19 * t1815 * t3118;
    let t23577 = t7884 * t8169;
    let t23588 = t555 * t1819 * t7909;
    let t23591 = t555 * t8185 * t7898;
    let t23622 = t24 * t20685;
    let t23625 = t555 * t23622 * t557 * t2988;
    let t23647 = t20075 * t13 * t2969;
    (t23572, t23575, t23577, t23588, t23591, t23622, t23625, t23647)
}
