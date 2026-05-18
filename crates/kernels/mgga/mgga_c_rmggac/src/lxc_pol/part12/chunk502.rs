//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 502/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk502<F: Float>(t4232: F, t4252: F, t4255: F, t4259: F, t4290: F, t4351: F, t5407: F, t5409: F, t5417: F, t5418: F, t5420: F, t5421: F, t5426: F, t5427: F, t5429: F, t5433: F) -> F {
    let t5458 = t5407 - t5409 + t5417 + t5418 + t5420 - t5421 + t4232 + t4252 - t4255 - t4259 + t5426 - t4351 + t5427 + t5429 + t4290 - t5433;
    t5458
}
