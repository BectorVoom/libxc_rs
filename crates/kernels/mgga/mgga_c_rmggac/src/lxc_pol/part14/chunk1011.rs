//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 1011/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk1011<F: Float>(t5204: F, t649: F, t8764: F, t5207: F, t5211: F, t7599: F, t5199: F, t5187: F, t5218: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t41209 = t649 * t5204;
    let t41210 = t8764 * t41209;
    let t41212 = t649 * t5207;
    let t41213 = t8764 * t41212;
    let t41215 = t649 * t5211;
    let t41216 = t7599 * t41215;
    let t41218 = t649 * t5199;
    let t41219 = t7599 * t41218;
    let t41221 = t649 * t5187;
    let t41222 = t7599 * t41221;
    let t41224 = t649 * t5218;
    let t41225 = t7599 * t41224;
    (t41209, t41210, t41212, t41213, t41215, t41216, t41218, t41219, t41221, t41222, t41224, t41225)
}
