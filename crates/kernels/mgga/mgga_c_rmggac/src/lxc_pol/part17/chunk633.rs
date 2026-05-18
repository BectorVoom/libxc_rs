//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 633/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk633<F: Float>(t1685: F, t71: F, t131: F, t638: F, t639: F, t2338: F, t356: F, t2164: F, t574: F, t1656: F, t640: F, t2402: F, t333: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t8849 = t71 * t1685;
    let t8850 = t8849 * t131;
    let t8852 = t638 * t639 * t8850;
    let t8854 = t2338 * t356;
    let t8856 = t638 * t639 * t8854;
    let t8858 = t2164 * t574;
    let t8860 = t638 * t639 * t8858;
    let t8862 = t640 * t1656;
    let t8864 = t638 * t639 * t8862;
    let t8866 = t2402 * t333;
    (t8849, t8850, t8852, t8854, t8856, t8858, t8860, t8862, t8864, t8866)
}
