//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 37/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk37<F: Float>(t88: F, t90: F, rho1: F, tau1: F) -> (F, F, F) {
    let t91 = t90 * t88;
    let t94 = pow_1_3::<f64>(rho1);
    let t95 = t94 * t94;
    let t97 = F::new(1.0) / t95 / rho1;
    let t98 = tau1 * t97;
    (t91, t95, t98)
}
