//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1185/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1185<F: Float>(t546: F, t8021: F, t565: F, t2182: F, t3303: F, t146: F, t6533: F, t774: F, t2147: F, t2150: F, t6856: F, t110: F, t252: F, t6359: F, t20200: F, t548: F) -> (F, F, F, F, F, F, F) {
    let t22780 = t546 * t8021;
    let t22783 = t565 * t8021;
    let t22790 = t2182 * t3303;
    let t22796 = t146 * t6533 * t774;
    let t22800 = t2147 * t6856 * t2150;
    let t22820 = t146 * t110 * t6359 * t252;
    let t22836 = t146 * t20200 * t548;
    (t22780, t22783, t22790, t22796, t22800, t22820, t22836)
}
