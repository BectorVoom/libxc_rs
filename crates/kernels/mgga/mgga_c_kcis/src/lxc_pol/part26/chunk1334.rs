//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1334/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1334<F: Float>(t28644: F, t5897: F, t22313: F, t27494: F, t22335: F, t27544: F, t20906: F, t97821: F, t22384: F, t7948: F, t5752: F, t5935: F) -> (F, F, F, F, F, F) {
    let t102860 = F::cast_from(2.0_f64) * t5897 * t28644;
    let t102864 = F::cast_from(4.0_f64) * t27494 * t22313;
    let t102867 = t27544 * t22335;
    let t102869 = t97821 * t20906;
    let t102871 = t7948 * t22384;
    let t102873 = t5752 * t5935;
    (t102860, t102864, t102867, t102869, t102871, t102873)
}
