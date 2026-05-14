//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 422/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk422<F: Float>(t197: F, t4388: F, t53: F, t57: F, t60: F, t62: F, t1173: F, t1175: F, t1240: F, t461: F, t1171: F, t225: F, t226: F) -> (F, F, F, F, F, F) {
    let t4389 = t197 * t4388;
    let t4394 = t53 * t53;
    let t4396 = 1.0 / t57 / t4394;
    let t4406 = t60 * t60;
    let t4408 = 1.0 / t62 / t4406;
    let t4435 = t1173 * t1175;
    let t4438 = t461 * t1240;
    let t4441 = t1171 * t225;
    let t4443 = 1.0 / t226 / t4441;
    (t4389, t4396, t4408, t4435, t4438, t4443)
}
