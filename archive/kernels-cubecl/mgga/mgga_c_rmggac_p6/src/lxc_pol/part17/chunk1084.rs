//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 1084/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk1084<F: Float>(t1528: F, t236: F, t615: F, t7230: F, t7231: F, t4044: F, t46055: F, t5058: F, t8639: F, t8642: F, t40759: F, t8646: F) -> (F, F, F, F) {
    let t47772 = t7230 * t7231 * t236 * t1528 * t615;
    let t47774 = t4044 * t46055;
    let t47785 = t5058 * t8639 * t8642;
    let t47787 = t40759 * t8646;
    (t47772, t47774, t47785, t47787)
}
