//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 39/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk39<F: Float>(t101: F, t99: F, t31: F, t36: F, t87: F, t91: F, t98: F) -> (F, F) {
    let t102 = t101 * t99;
    let t107 = F::cast_from(2.0_f64) * t87 * t91 + F::cast_from(2.0_f64) * t98 * t102 - t31 * t36 / F::cast_from(4.0_f64);
    (t102, t107)
}
