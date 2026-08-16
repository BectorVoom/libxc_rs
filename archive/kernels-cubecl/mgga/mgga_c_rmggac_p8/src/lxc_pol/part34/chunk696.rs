//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 696/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk696<F: Float>(t1326: F, t69239: F, t13916: F, t2048: F, t3851: F, t328: F, t3814: F, t2566: F, t13940: F, t1330: F, t793: F, t851: F) -> (F, F, F, F, F, F, F, F, F) {
    let t69240 = t1326 * t69239;
    let t69241 = t13916 * t69240;
    let t69243 = t3851 * t2048;
    let t69244 = t69243 * t328;
    let t69245 = F::cast_from(0.36366215538993788972e-1_f64) * t69244;
    let t69249 = t3814 * t2048;
    let t69250 = t69249 * t2566;
    let t69261 = t13940 * t69240;
    let t69265 = t793 * t1330;
    let t69267 = t851 * t1330;
    (t69240, t69241, t69243, t69245, t69249, t69250, t69261, t69265, t69267)
}
