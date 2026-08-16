//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2436/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2436(t10771: f64, t14271: f64, t14276: f64, t17535: f64, t17538: f64, t17541: f64, t21259: f64, t2886: f64, t4433: f64, t49430: f64, t5743: f64, t69288: f64, t69291: f64, t69294: f64, t69297: f64, t69299: f64, t69302: f64, t69305: f64, t69307: f64, t69310: f64, t69313: f64, t931: f64) -> f64 {
    let t69326 = t69288 + t69291 - t69294 - t69297 + t69299 + t69302 + t69305 - t69307 - t69310 - t69313 + 18.0_f64 * t14271 * t17535 - 12.0_f64 * t14276 * t17538 - 0.57895126195293126241e3_f64 * t49430 * t17541 - 24.0_f64 * t10771 * t21259 * t931 + 18.0_f64 * t2886 * t5743 * t4433;
    t69326
}
