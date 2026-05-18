//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 947/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk947<F: Float>(t40262: F, t16504: F, t34975: F, t552: F, t7455: F, t16503: F, t34962: F, t7461: F, t22971: F, t7467: F, t1965: F, t1967: F, t28: F, t8511: F) -> (F, F, F, F, F) {
    let t40263 = F::new(0.39726959900411316772e-4) * t40262;
    let t40266 = t34975 * t16504 * t552 * t7455;
    let t40270 = t16503 * t34962 * t552 * t7461;
    let t40274 = t16503 * t22971 * t552 * t7467;
    let t40278 = t8511 * t1965 * t1967 * t28;
    (t40263, t40266, t40270, t40274, t40278)
}
