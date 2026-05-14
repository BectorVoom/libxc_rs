//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1414/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1414<F: Float>(t32433: F, t33873: F, t115026: F, t9516: F, t113853: F, t113857: F, t109565: F, t109567: F, t109570: F, t109575: F, t109577: F, t109580: F, t113851: F, t113855: F, t32338: F, t32350: F, t32376: F, t32461: F, t33784: F, t33794: F, t33941: F) -> (F,) {
    let t115240 = t32433 * t33873;
    let t115247 = t9516 * t115026;
    let t115251 = 0.15476481481481481481e-2 * t113853;
    let t115253 = 0.15476481481481481481e-2 * t113857;
    let t115256 = -0.23148148148148148148e-2 * t33941 * t32350 + 0.62081666666666666667e-2 * t32376 * t32338 * t33784 - 0.35740740740740740742e-2 * t115240 + 0.34722222222222222222e-2 * t109565 + 0.34722222222222222222e-2 * t109567 + 0.17361111111111111111e-2 * t109570 + 0.34722222222222222222e-2 * t109575 + 0.34722222222222222222e-2 * t109577 - 0.44675925925925925927e-3 * t115247 + 0.17361111111111111111e-2 * t109580 + 0.69644166666666666664e-2 * t113851 - t115251 + 0.46429444444444444443e-2 * t113855 - t115253 + 0.34722222222222222222e-2 * t33794 * t32461;
    (t115256,)
}
