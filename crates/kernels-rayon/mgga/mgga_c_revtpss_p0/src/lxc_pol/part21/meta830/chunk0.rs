//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3094/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3094(t11239: f64, t1770: f64, t13061: f64, t17225: f64, t3647: f64, t11262: f64, t1261: f64, t5303: f64, t3711: f64, t5298: f64, t127: f64, t17352: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t56730 = t1770 * t11239;
    let t56731 = t56730 * t13061;
    let t56734 = t3647 * t17225;
    let t56739 = t1261 * t11262 * t5303;
    let t56740 = 0.15879265969655458138e-3_f64 * t56739;
    let t56742 = t3711 * t11262 * t5298;
    let t56756 = t127 * t17352;
    (t56730, t56731, t56734, t56740, t56742, t56756)
}
