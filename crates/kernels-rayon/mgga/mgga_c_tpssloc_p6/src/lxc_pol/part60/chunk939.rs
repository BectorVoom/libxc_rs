//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 939/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk939(t32827: f64, t6547: f64, t23168: f64, t32819: f64, t234: f64, t7510: f64, t23110: f64, t23185: f64, t32822: f64, t2717: f64, t7537: f64, t112943: f64, t6562: f64, t7488: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t118738 = t6547 * t32827;
    let t118744 = t23168 * t32819;
    let t118747 = t234 * t7510;
    let t118766 = t23185 * t23110 * t32822;
    let t118821 = t2717 * t7537;
    let t118830 = t6562 * t112943 * t7488;
    (t118738, t118744, t118747, t118766, t118821, t118830)
}
