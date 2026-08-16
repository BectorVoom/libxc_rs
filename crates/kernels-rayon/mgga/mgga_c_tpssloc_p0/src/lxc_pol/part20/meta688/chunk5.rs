//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2610/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2610(t1227: f64, t13969: f64, t15611: f64, t15454: f64, t4973: f64, t49850: f64, t11662: f64, t11665: f64, t15478: f64, t15737: f64, t44985: f64, t44988: f64, t44991: f64, t44994: f64, t44996: f64, t4582: f64, t48497: f64, t4950: f64, t51002: f64) -> f64 {
    let t53023 = t1227 * t13969 * t15611;
    let t53026 = t1227 * t13969 * t15454;
    let t53033 = t1227 * t49850 * t4973;
    let t53034 = t53033 / 3456.0_f64;
    let t53037 = -t44985 / 2304.0_f64 - t44988 / 2304.0_f64 - t44991 / 1152.0_f64 - t44994 / 1152.0_f64 + 5.0_f64 / 384.0_f64 * t1227 * t4582 * t51002 * t48497 - t53023 / 1152.0_f64 - 5.0_f64 / 2592.0_f64 * t53026 - t44996 * t4950 / 1536.0_f64 - t11665 * t15478 / 768.0_f64 + t53034 + t15737 * t11662 / 512.0_f64;
    t53037
}
