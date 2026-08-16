//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1012/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1012<F: Float>(t1535: F, t2869: F, t1539: F, t2889: F, t1145: F, t1555: F, t2876: F, t1161: F, t3676: F, t7785: F, t3683: F, t647: F, tau1: F) -> (F, F, F, F, F, F, F, F) {
    let t9468 = t1535 * t2869;
    let t9474 = t1539 * t2889;
    let t9475 = t1145 * t9474;
    let t9478 = t1555 * t2876;
    let t9479 = t1161 * t9478;
    let t9482 = t3676 * t7785;
    let t9485 = t3683 * t7785;
    let t9488 = t647 * tau1;
    (t9468, t9474, t9475, t9478, t9479, t9482, t9485, t9488)
}
