//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 870/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk870(t14117: f64, t21708: f64, t8807: f64, t8811: f64, t15333: f64, t68528: f64, t13823: f64, t38848: f64, t7756: f64, t2415: f64, t70504: f64, t7349: f64) -> (f64, f64, f64, f64, f64) {
    let t75561 = t21708 * t14117 * t8807;
    let t75564 = t21708 * t14117 * t8811;
    let t75566 = t68528 * t15333;
    let t75572 = t13823 * t38848 * t7756;
    let t75575 = t7349 * t2415 * t70504;
    (t75561, t75564, t75566, t75572, t75575)
}
