//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1290/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1290<F: Float>(t2517: F, t3534: F, t9440: F, t11498: F, t23263: F, t11497: F, t2511: F, t7403: F, t23235: F, t23238: F, t2475: F, t4292: F, t27070: F, t3499: F, t9449: F, t9582: F) -> (F, F, F, F, F, F) {
    let t31631 = 0.32163958997385070134e2 * t2517 * t3534 * t9440;
    let t31633 = 0.1034520258385468006e4 * t23263 * t11498;
    let t31636 = 0.51726012919273400301e3 * t7403 * t11497 * t2511;
    let t31640 = 0.24955700379505800916e5 * t23235 * t4292 * t23238 * t2475;
    let t31642 = 8.0 * t27070 * t3499;
    let t31644 = 8.0 * t9449 * t9582;
    (t31631, t31633, t31636, t31640, t31642, t31644)
}
