//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1296/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1296<F: Float>(t4350: F, t539: F, t6174: F, t1588: F, t1597: F, t32457: F, t964: F, t1310: F, t1589: F, t33870: F, t9529: F, t114596: F, t114606: F, t114628: F, t114633: F, t114635: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t115710 = t539 * t4350;
    let t115711 = t6174 * t115710;
    let t115725 = t1588 * t1597;
    let t115726 = t6174 * t115725;
    let t115750 = t964 * t32457;
    let t115772 = t1310 * t1589;
    let t115806 = t9529 * t33870;
    let t115817 = 0.23214722222222222222e-2 * t114596;
    let t115819 = 0.15476481481481481481e-2 * t114606;
    let t115828 = 0.61905925925925925925e-2 * t114628;
    let t115831 = 0.15476481481481481481e-2 * t114633;
    let t115846 = 0.23214722222222222222e-2 * t114635;
    (t115710, t115711, t115725, t115726, t115750, t115772, t115806, t115817, t115819, t115828, t115831, t115846)
}
