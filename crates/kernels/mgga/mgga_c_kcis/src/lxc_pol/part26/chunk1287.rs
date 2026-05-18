//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1287/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1287<F: Float>(t1394: F, t22275: F, t28503: F, t102029: F, t102032: F, t102035: F, t102038: F, t7978: F, t98193: F, t99098: F, t99100: F, t99108: F, t99117: F, t99129: F, t99131: F) -> (F, F) {
    let t102041 = t1394 * t28503 * t22275;
    let t102045 = -F::new(0.69505208333333333334e-3) * t7978 * t102029 - t99098 - t99100 + t99108 - t99117 + F::new(0.7722800925925925926e-4) * t102032 + F::new(0.7722800925925925926e-4) * t102035 - F::new(0.25794135802469135802e-3) * t102038 + F::new(0.23214722222222222221e-2) * t102041 - F::new(0.41270617283950617283e-2) * t98193 + t99129 - F::new(0.15445601851851851852e-3) * t99131;
    (t102041, t102045)
}
