//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 973/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk973<F: Float>(t9444: F, t958: F, t2512: F, t3494: F, t1390: F, t2473: F, t2476: F, t3574: F, t986: F, t2590: F, t3577: F, t372: F, t9296: F, t9396: F, t9402: F, t9404: F, t9411: F, t9415: F, t9443: F, t996: F) -> (F, F, F, F, F, F) {
    let t9446 = 2.0 * t9444 * t958;
    let t9448 = 1.0 * t3494 * t2512;
    let t9449 = t1390 * t2473;
    let t9451 = 2.0 * t9449 * t2476;
    let t9452 = t3574 * t986;
    let t9457 = -0.19751673498613801407e-1 * t9396 - 0.310907e-1 * t9411 * t372 + t9296 - t9402 - t9404 - t9415 - t9443 - t9446 - t9448 + t9451 + 0.11696447245269292414e1 * t9452 * t996 + 0.5848223622634646207e0 * t3577 * t2590;
    (t9446, t9448, t9449, t9451, t9452, t9457)
}
