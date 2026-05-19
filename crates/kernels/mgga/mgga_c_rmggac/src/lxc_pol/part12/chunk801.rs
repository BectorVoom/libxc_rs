//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 801/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk801<F: Float>(t38200: F, t38203: F, t38204: F, t38205: F, t38206: F, t7267: F, t7270: F, t7277: F, t7280: F, t7286: F, t8040: F, t8397: F) -> (F, F) {
    let t38207 = t38200 + t7267 + F::cast_from(0.36366215538993788972e-1_f64) * t7270 + t7277 + F::cast_from(0.14546486215597515589e0_f64) * t7280 + t7286 - t8040 + t38203 - t38204 - t38205 + t38206;
    let t38210 = F::cast_from(0.47896966807455234256e0_f64) * t8397;
    (t38207, t38210)
}
