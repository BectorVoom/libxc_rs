//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 805/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk805(t69971: f64, t9222: f64, t27: f64, t8455: f64, t16129: f64, t70489: f64, t201: f64, t209: f64, t457: f64, t68504: f64, t68505: f64, t8440: f64) -> (f64, f64, f64) {
    let t74468 = 0.1064114997332445985e-4_f64 * t9222 * t69971;
    let t74469 = t27 * t8455;
    let t74471 = t70489 * t16129 * t74469;
    let t74477 = t68504 * t68505 * t8440 * t209 * t457 * t201;
    (t74468, t74471, t74477)
}
