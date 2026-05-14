//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 767/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk767<F: Float>(t1894: F, t5447: F, t5448: F, t1647: F, t1898: F, t1907: F, t1945: F, t61: F, t1719: F, t695: F) -> (F, F, F, F) {
    let t5451 = 0.62071215503128080361e4 * t5447 * t1894 * t5448;
    let t5454 = 0.28947563097646563121e3 * t1907 * t1898 * t1647;
    let t5455 = t61 * t1945;
    let t5456 = t1719 * t695;
    (t5451, t5454, t5455, t5456)
}
