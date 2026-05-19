//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 808/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk808<F: Float>(t290: F, t9030: F, t118: F, t128: F, t1494: F, t1986: F, t209: F, t1550: F, t5144: F, t7778: F, t5267: F, t903: F) -> (F, F, F, F) {
    let t39507 = t290 * t9030;
    let t39513 = t1986 * t118 * t128 * t1494 * t209;
    let t39528 = t1550 * t7778 * t5144;
    let t39529 = F::cast_from(0.15965655602485078085e0_f64) * t39528;
    let t39535 = t903 * t7778 * t5267;
    (t39507, t39513, t39529, t39535)
}
