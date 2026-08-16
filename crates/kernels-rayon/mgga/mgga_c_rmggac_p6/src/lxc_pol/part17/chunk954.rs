//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 954/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk954(t38530: f64, t9165: f64, t40771: f64, t8457: f64, t36596: f64, t9831: f64, t36457: f64, t9835: f64, t1469: f64, t3369: f64, t39851: f64, t559: f64) -> (f64, f64, f64, f64, f64) {
    let t45825 = t38530 * t9165;
    let t45827 = t40771 * t8457;
    let t45830 = t36596 * t9831;
    let t45832 = t36457 * t9835;
    let t45836 = t39851 * t3369 * t559 * t1469;
    (t45825, t45827, t45830, t45832, t45836)
}
