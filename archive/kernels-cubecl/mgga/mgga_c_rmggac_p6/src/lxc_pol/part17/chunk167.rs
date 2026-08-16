//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 167/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk167<F: Float>(t537: F, t90: F, t95: F, t101: F, t102: F, t320: F, t87: F, t98: F, rho1: F, tau1: F) -> (F, F, F, F, F, F) {
    let t538 = t90 * t537;
    let t541 = rho1 * rho1;
    let t543 = F::cast_from(1.0_f64) / t95 / t541;
    let t544 = tau1 * t543;
    let t547 = -t537;
    let t548 = t101 * t547;
    let t551 = F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t87 * t538 - F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t544 * t102 + F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t98 * t548 + t320;
    (t538, t541, t544, t547, t548, t551)
}
