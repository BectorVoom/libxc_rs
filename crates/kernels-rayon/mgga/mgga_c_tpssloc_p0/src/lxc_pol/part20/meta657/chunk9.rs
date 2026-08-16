//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2437/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2437(t1041: f64, t4589: f64, t49850: f64, t10969: f64, t41687: f64, t42600: f64, t42721: f64, t42729: f64, t42731: f64, t4582: f64, t4583: f64, t4588: f64, t45993: f64, t4600: f64, t46006: f64, t48497: f64, t49827: f64, t49829: f64, t49832: f64, t49846: f64) -> f64 {
    let t49852 = t1041 * t49850 * t4589;
    let t49853 = 5.0_f64 / 20736.0_f64 * t49852;
    let t49854 = t10969 * t41687;
    let t49860 = -t42721 / 2304.0_f64 + t42729 / 2304.0_f64 + 19.0_f64 / 1296.0_f64 * t49827 - t49829 / 216.0_f64 + t49832 - t1041 * t4582 * t4583 * t46006 / 768.0_f64 + 5.0_f64 / 4608.0_f64 * t1041 * t4582 * t4588 * t46006 - t1041 * t4582 * t4583 * t45993 / 2304.0_f64 - 5.0_f64 / 1152.0_f64 * t49846 - 19.0_f64 / 576.0_f64 * t42600 * t4600 - t49853 - 5.0_f64 / 432.0_f64 * t1041 * t4582 * t49854 * t48497 + t42731 / 288.0_f64;
    t49860
}
