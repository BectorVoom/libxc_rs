//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1334/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1334(t28644: f64, t5897: f64, t22313: f64, t27494: f64, t22335: f64, t27544: f64, t20906: f64, t97821: f64, t22384: f64, t7948: f64, t5752: f64, t5935: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t102860 = 2.0_f64 * t5897 * t28644;
    let t102864 = 4.0_f64 * t27494 * t22313;
    let t102867 = t27544 * t22335;
    let t102869 = t97821 * t20906;
    let t102871 = t7948 * t22384;
    let t102873 = t5752 * t5935;
    (t102860, t102864, t102867, t102869, t102871, t102873)
}
