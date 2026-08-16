//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1047/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1047(t4942: f64, t500: f64, t1489: f64, t16204: f64, t16207: f64, t482: f64, t170: f64, t50: f64, t65: f64, t16200: f64, t16202: f64, t16205: f64, t16208: f64, t16210: f64, t16215: f64) -> (f64, f64, f64, f64, f64) {
    let t16217 = t4942 * t500;
    let t16219 = t1489 * t16204;
    let t16221 = t482 * t16207;
    let t16224 = t65 * t50 * t170;
    let t16226 = -0.28769444444444444444e1_f64 * t16200 + 0.27618666666666666667e2_f64 * t16202 - 0.10229135802469135803e2_f64 * t16205 + 0.89504938271604938273e1_f64 * t16208 + 0.31310740740740740741e1_f64 * t16210 + 0.366775e-1_f64 * t16215 - 0.58684e0_f64 * t16217 + 0.65204444444444444445e0_f64 * t16219 + 0.5705388888888888889e0_f64 * t16221 + 0.13490888888888888889e1_f64 * t16224;
    (t16217, t16219, t16221, t16224, t16226)
}
