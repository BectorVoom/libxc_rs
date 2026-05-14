//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1360/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1360<F: Float>(t2864: F, t11963: F, t11928: F, t11929: F, t1795: F, t11934: F, t3732: F, t3742: F, t11958: F, t11717: F, t9856: F, t9901: F, t10177: F, t16107: F, t28833: F, t33063: F, t3685: F, t3771: F, t7897: F, t7903: F, t9826: F, t9868: F, t9873: F, t9893: F, tau1: F) -> (F, F, F) {
    let t33439 = t2864 * tau1;
    let t33440 = t11963 * t33439;
    let t33444 = t11928 * t11929 * t1795;
    let t33447 = t11934 * t1795;
    let t33448 = t3732 * t33447;
    let t33453 = t3742 * t33447;
    let t33460 = t11958 * t1795;
    let t33461 = t3742 * t33460;
    let t33472 = t11717 * t9856;
    let t33475 = t11717 * t9901;
    let t33478 = -0.1152e-2 * t16107 * t33063 - 0.47407407407407407408e0 * t9826 * t33440 - 0.33792e-7 * t3685 * t33444 + 0.6336e-2 * t7903 * t33448 - 0.47407407407407407408e0 * t9893 * t33440 - 0.18773333333333333333e-2 * t9826 * t33453 - 0.33792e-7 * t3771 * t33444 + 0.704e-3 * t7897 * t33448 + 0.18773333333333333333e-2 * t9893 * t33461 - 0.5632e-2 * t9873 * t33453 - 0.14222222222222222222e1 * t9873 * t33440 + 0.5632e-2 * t9868 * t33461 - 0.14222222222222222222e1 * t9868 * t33440 + 0.1152e-2 * t28833 * t33472 + 0.5632e-5 * t10177 * t33475;
    (t33472, t33475, t33478)
}
