//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3213/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3213(t19270: f64, t3633: f64, t4700: f64, t63765: f64, t63767: f64, t63769: f64, t63771: f64, t63829: f64, t64100: f64, t64253: f64, t64259: f64, t64433: f64, t65290: f64, t65293: f64, t65296: f64, t65299: f64) -> f64 {
    let t66891 = 2.0_f64 * t19270 * t3633 * t4700 + t63765 - t63767 + t63769 + t63771 - t63829 + t64100 + t64253 - t64259 + t64433 - t65290 - t65293 + t65296 - t65299;
    t66891
}
