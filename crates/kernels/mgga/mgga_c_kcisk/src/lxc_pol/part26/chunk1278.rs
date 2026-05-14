//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1278/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1278<F: Float>(t20149: F, t33476: F, t9446: F, t1292: F, t1308: F, t20886: F, t32019: F, t33593: F, t33376: F, t3969: F, t32176: F, t33377: F, t33352: F, t3748: F, t33349: F, t113962: F, t32008: F) -> (F, F, F, F, F, F, F, F, F) {
    let t114095 = t9446 * t20149 * t33476;
    let t114107 = t20886 * t1292 * t1308;
    let t114111 = 0.69444444444444444446e-2 * t32019 * t33593;
    let t114121 = t33376 * t3969;
    let t114125 = 0.26805555555555555556e-2 * t33377 * t32176;
    let t114136 = t3748 * t33352;
    let t114138 = t3748 * t33349;
    let t114139 = 0.14739506172839506172e-2 * t114138;
    let t114157 = t32008 * t113962;
    (t114095, t114107, t114111, t114121, t114125, t114136, t114138, t114139, t114157)
}
