//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1292/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1292<F: Float>(t26998: F, t3535: F, t9400: F, t9592: F, t9571: F, t27001: F, t9575: F, t11478: F, t7371: F, t2511: F, t2517: F, t4293: F, t2475: F, t4322: F, t7403: F, t11481: F, t7415: F) -> (F, F, F, F, F, F, F, F) {
    let t31652 = 0.64327917994770140268e2 * t26998 * t3535;
    let t31654 = 0.64327917994770140268e2 * t9400 * t9592;
    let t31656 = 0.32163958997385070134e2 * t9400 * t9571;
    let t31658 = 0.1034520258385468006e4 * t27001 * t9575;
    let t31660 = 12.0 * t7371 * t11478;
    let t31663 = 6.0 * t2517 * t4293 * t2511;
    let t31666 = 0.57895126195293126241e3 * t7403 * t4322 * t2475;
    let t31668 = 8.0 * t7415 * t11481;
    (t31652, t31654, t31656, t31658, t31660, t31663, t31666, t31668)
}
