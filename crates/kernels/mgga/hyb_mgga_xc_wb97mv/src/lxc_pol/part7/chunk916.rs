//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 916/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk916<F: Float>(t1175: F, t1227: F, t1829: F, t19: F, t1975: F, t2966: F, t2968: F, t3003: F, t3132: F, t3136: F, t546: F, t554: F, t8185: F, t8188: F, t8193: F, t8196: F, t8198: F, t8208: F, t8211: F, t8431: F, t8436: F, t8440: F) -> (F,) {
    let t8443 = t8185 / 96.0 - t554 * t3003 * t8188 / 32.0 - t8193 + 7.0 / 32.0 * t8196 + t8198 / 96.0 - 3.0 / 64.0 * t1975 * t1227 - 3.0 / 32.0 * t546 * t3132 - 3.0 / 32.0 * t546 * t3136 - t8208 - t8211 - 3.0 / 64.0 * t19 * t8431 - 3.0 / 64.0 * t1175 * t1829 - 3.0 / 16.0 * t2966 * t8436 - 3.0 / 32.0 * t8440 * t2968;
    (t8443,)
}
