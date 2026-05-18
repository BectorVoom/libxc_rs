//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1273/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1273<F: Float>(t4189: F, t7397: F, t7962: F, t1505: F, t29412: F, t1555: F, t22310: F, t94833: F, t29424: F, t39301: F, t22300: F, t17311: F, t28576: F) -> (F, F, F, F, F, F) {
    let t101826 = F::new(2.0) * t4189 * t7962 * t7397;
    let t101827 = t29412 * t1505;
    let t101828 = t101827 * t1555;
    let t101830 = F::new(6.0) * t94833 * t22310;
    let t101832 = F::new(6.0) * t39301 * t29424;
    let t101833 = t22300 * t7962;
    let t101835 = F::new(4.0) * t17311 * t28576;
    (t101826, t101828, t101830, t101832, t101833, t101835)
}
