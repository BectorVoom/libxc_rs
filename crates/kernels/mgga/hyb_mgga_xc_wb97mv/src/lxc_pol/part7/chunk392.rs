//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 392/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk392<F: Float>(t132: F, t1382: F, t1439: F, t338: F, t392: F, t1289: F, t400: F, t196: F, t408: F, t397: F, t296: F, t1296: F, t195: F, t405: F, t407: F, t313: F, t209: F, t316: F, dens_threshold: F, rho1: F, sigma2: F, tau1: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t133 = t132 <= zeta_threshold;
    let t134 = rho1 <= dens_threshold || t133;
    let t1443 = piecewise3(t134, 0.0, t1382 * t392 / 2.0 + t338 * t1439 / 2.0);
    let t1445 = t400 * t1289;
    let t1447 = 1.0 / t196 / t1445;
    let t1448 = t1447 * t408;
    let t1451 = t397 * sigma2;
    let t1452 = t296 * t1451;
    let t1453 = t400 * t1296;
    let t1455 = 1.0 / t195 / t1453;
    let t1457 = 1.0 / t407 / t405;
    let t1458 = t1455 * t1457;
    let t1461 = t313 * tau1;
    let t1466 = t316 * t209;
    (t1443, t1447, t1448, t1451, t1452, t1455, t1457, t1458, t1461, t1466)
}
