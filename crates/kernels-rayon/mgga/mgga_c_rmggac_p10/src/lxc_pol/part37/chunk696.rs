//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 696/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk696(t1326: f64, t69239: f64, t13916: f64, t2048: f64, t3851: f64, t328: f64, t3814: f64, t2566: f64, t13940: f64, t1330: f64, t793: f64, t851: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t69240 = t1326 * t69239;
    let t69241 = t13916 * t69240;
    let t69243 = t3851 * t2048;
    let t69244 = t69243 * t328;
    let t69245 = 0.36366215538993788972e-1_f64 * t69244;
    let t69249 = t3814 * t2048;
    let t69250 = t69249 * t2566;
    let t69261 = t13940 * t69240;
    let t69265 = t793 * t1330;
    let t69267 = t851 * t1330;
    (t69240, t69241, t69243, t69245, t69249, t69250, t69261, t69265, t69267)
}
