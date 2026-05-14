//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1040/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1040<F: Float>(t10621: F, t687: F, t2044: F, t3979: F, t154: F, t3994: F, t3975: F, t719: F, t157: F, t723: F, t160: F, t727: F, t163: F, t731: F, t166: F, t735: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t10622 = t687 * t10621;
    let t10626 = t2044 * t3979;
    let t10633 = t154 * t3994;
    let t10636 = t719 * t3975;
    let t10641 = t157 * t3994;
    let t10644 = t723 * t3975;
    let t10649 = t160 * t3994;
    let t10652 = t727 * t3975;
    let t10657 = t163 * t3994;
    let t10660 = t731 * t3975;
    let t10665 = t166 * t3994;
    let t10668 = t735 * t3975;
    (t10622, t10626, t10633, t10636, t10641, t10644, t10649, t10652, t10657, t10660, t10665, t10668)
}
