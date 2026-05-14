//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 901/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk901<F: Float>(t3130: F, t667: F, t26: F, t1225: F, t1827: F, t2967: F, t668: F, t1175: F, t1836: F, t1839: F, t19: F, t1967: F, t1971: F, t2966: F, t2987: F, t2989: F, t2991: F, t8126: F, t8131: F, t8135: F, t8140: F, t8142: F, t8143: F, t8148: F, t8150: F, t8158: F, t8160: F) -> (F, F, F, F, F, F, F, F) {
    let t8164 = t3130 * t667;
    let t8165 = t26 * t8164;
    let t8168 = t1225 * t1827;
    let t8169 = t26 * t8168;
    let t8172 = t2967 * t668;
    let t8176 = t1175 * t1836 / 32.0;
    let t8178 = t1175 * t1839 / 32.0;
    let t8183 = -t2987 * t8126 * t2991 / 24.0 - t2987 * t2989 * t8131 / 24.0 - t2987 * t2989 * t8135 / 48.0 - 7.0 / 144.0 * t8140 * t8142 * t8143 - t2987 * t8148 * t8150 / 12.0 - t8158 + t8160 * t2989 * t8143 / 16.0 - 3.0 / 32.0 * t19 * t8165 - 3.0 / 64.0 * t19 * t8169 - 3.0 / 16.0 * t2966 * t8172 - t8176 - t8178 - 3.0 / 64.0 * t1175 * t1967 - 3.0 / 32.0 * t1175 * t1971;
    (t8164, t8165, t8168, t8169, t8172, t8176, t8178, t8183)
}
