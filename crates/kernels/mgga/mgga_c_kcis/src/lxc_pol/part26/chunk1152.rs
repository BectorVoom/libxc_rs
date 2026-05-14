//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1152/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1152<F: Float>(t2104: F, t27614: F, t6176: F, t6188: F, t101894: F, t27583: F, t29532: F, t4425: F, t7978: F, t1394: F, t21898: F, t7923: F, t22275: F, t28503: F, t98193: F, t99098: F, t99100: F, t99108: F, t99117: F, t99129: F, t99131: F) -> (F, F, F, F) {
    let t102029 = t6176 * t27614 * t2104 * t6188;
    let t102032 = t27583 * t101894;
    let t102035 = t7978 * t4425 * t29532;
    let t102038 = t1394 * t7923 * t21898;
    let t102041 = t1394 * t28503 * t22275;
    let t102045 = -0.69505208333333333334e-3 * t7978 * t102029 - t99098 - t99100 + t99108 - t99117 + 0.7722800925925925926e-4 * t102032 + 0.7722800925925925926e-4 * t102035 - 0.25794135802469135802e-3 * t102038 + 0.23214722222222222221e-2 * t102041 - 0.41270617283950617283e-2 * t98193 + t99129 - 0.15445601851851851852e-3 * t99131;
    (t102029, t102038, t102041, t102045)
}
