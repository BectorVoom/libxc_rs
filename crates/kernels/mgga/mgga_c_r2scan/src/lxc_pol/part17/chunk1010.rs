//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1010/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1010<F: Float>(t40217: F, t40222: F, t40232: F, t40234: F, t40241: F, t40243: F, t40257: F, t40260: F, t3718: F, t5086: F, t11002: F, t1115: F, t2847: F, t40781: F, t40797: F, t40804: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t41749 = 0.21951497276451705328e-1 * t40217;
    let t41751 = 0.46230515946956099004e0 * t40222;
    let t41756 = 0.39029762157531132074e-1 * t40232;
    let t41757 = 0.11708928647259339622e0 * t40234;
    let t41762 = 0.93149212406257582492e-1 * t40241;
    let t41763 = 0.39029762157531132074e-1 * t40243;
    let t41775 = 0.21951497276451705328e-1 * t40257;
    let t41776 = 0.27944763721877274748e0 * t40260;
    let t41791 = t5086 * t3718;
    let t41816 = t11002 * t1115 * t2847;
    let t41859 = 8.0 / 3.0 * t40781;
    let t41867 = 8.0 / 3.0 * t40797;
    let t41870 = 8.0 / 3.0 * t40804;
    (t41749, t41751, t41756, t41757, t41762, t41763, t41775, t41776, t41791, t41816, t41859, t41867, t41870)
}
