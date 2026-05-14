//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1227/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1227<F: Float>(t10518: F, t21750: F, t21753: F, t21755: F, t21759: F, t21761: F, t21793: F, t21796: F, t21799: F, t21887: F, t21890: F, t25498: F, t25512: F, t25542: F, t25545: F, t25548: F, t25558: F, t25565: F, t29126: F, t546: F, t8148: F, t8160: F) -> (F,) {
    let t29873 = -5.0 / 144.0 * t21750 - 5.0 / 144.0 * t21753 + t21755 / 96.0 + t21759 / 48.0 + t21761 / 48.0 + t25498 / 54.0 - t25512 / 36.0 - 5.0 / 432.0 * t21793 + t21796 / 144.0 + t21799 / 144.0 + t21887 / 216.0 + t21890 / 288.0 - t25542 / 18.0 + t25545 / 24.0 - t25548 / 72.0 - 7.0 / 216.0 * t25558 - t25565 / 36.0 - 3.0 / 32.0 * t546 * t10518 + t8160 * t8148 * t29126 / 2.0;
    (t29873,)
}
