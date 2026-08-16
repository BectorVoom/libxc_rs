//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2708/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2708(t2319: f64, t670: f64, t10259: f64, t94: f64, t14619: f64, t750: f64, t4398: f64, t9372: f64, t39423: f64, t39425: f64, t39433: f64, t39436: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t49851 = t2319 * t670;
    let t49856 = t94 * t10259;
    let t49864 = t14619 * t750;
    let t49865 = 3.0_f64 * t49864;
    let t49866 = t4398 * t9372;
    let t49867 = 0.10254018858216406658e4_f64 * t49866;
    let t49868 = 0.65061487801810439052e-1_f64 * t39423;
    let t49869 = 0.97592231702715658578e-1_f64 * t39425;
    let t49870 = 0.14447919941302971323e1_f64 * t39433;
    let t49872 = 0.32530743900905219526e-1_f64 * t39436;
    (t49851, t49856, t49865, t49867, t49868, t49869, t49870, t49872)
}
