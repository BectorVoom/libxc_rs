//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2430/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2430(t13965: f64, t3114: f64, t14202: f64, t3117: f64, t10423: f64, t13995: f64, t10413: f64, t10422: f64, t14221: f64, t10949: f64, t14025: f64, t10195: f64, t10408: f64, t10433: f64, t10965: f64, t13991: f64, t14215: f64, t14511: f64, t1616: f64, t3070: f64, t42541: f64, t42565: f64, t42570: f64, t42586: f64, t42861: f64, t4596: f64, t4636: f64, t47679: f64, t973: f64) -> f64 {
    let t49690 = t3114 * t13965;
    let t49691 = t49690 / 4608.0_f64;
    let t49692 = t3117 * t14202;
    let t49693 = t49692 / 6912.0_f64;
    let t49697 = t13995 * t10423;
    let t49702 = t10413 * t10422 * t14221;
    let t49716 = t10949 * t14025;
    let t49718 = -t42586 / 2304.0_f64 - t49691 - t49693 + 35.0_f64 / 972.0_f64 * t973 * t42861 * t47679 + t49697 / 1152.0_f64 + t10965 * t4636 / 1536.0_f64 - t49702 / 1152.0_f64 + 5.0_f64 / 4608.0_f64 * t3070 * t10408 * t1616 * t10195 + t42541 * t14215 / 384.0_f64 - t14511 * t10433 / 1024.0_f64 + t42565 * t13991 / 32.0_f64 - t42570 * t4596 / 48.0_f64 + t49716 / 384.0_f64;
    t49718
}
