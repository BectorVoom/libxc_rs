//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 801/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk801(t38200: f64, t38203: f64, t38204: f64, t38205: f64, t38206: f64, t7267: f64, t7270: f64, t7277: f64, t7280: f64, t7286: f64, t8040: f64, t8397: f64) -> (f64, f64) {
    let t38207 = t38200 + t7267 + 0.36366215538993788972e-1_f64 * t7270 + t7277 + 0.14546486215597515589e0_f64 * t7280 + t7286 - t8040 + t38203 - t38204 - t38205 + t38206;
    let t38210 = 0.47896966807455234256e0_f64 * t8397;
    (t38207, t38210)
}
