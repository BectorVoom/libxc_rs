//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 927/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk927(t23110: f64, t23185: f64, t32822: f64, t2717: f64, t7537: f64, t112943: f64, t6562: f64, t7488: f64, t32792: f64, t6547: f64, t23204: f64, t32866: f64) -> (f64, f64, f64, f64, f64) {
    let t118766 = t23185 * t23110 * t32822;
    let t118821 = t2717 * t7537;
    let t118830 = t6562 * t112943 * t7488;
    let t118858 = t6547 * t32792;
    let t118885 = t6562 * t23204 * t32866;
    (t118766, t118821, t118830, t118858, t118885)
}
