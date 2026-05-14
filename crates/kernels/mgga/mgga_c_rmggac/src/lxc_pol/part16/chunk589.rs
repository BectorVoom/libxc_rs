//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 589/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk589<F: Float>(t2338: F, t356: F, t638: F, t639: F, t2164: F, t574: F, t1656: F, t640: F, t2298: F, t4601: F, t2301: F, t2604: F, t1614: F, t645: F, t903: F, t7844: F, t8642: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t8854 = t2338 * t356;
    let t8856 = t638 * t639 * t8854;
    let t8858 = t2164 * t574;
    let t8860 = t638 * t639 * t8858;
    let t8862 = t640 * t1656;
    let t8864 = t638 * t639 * t8862;
    let t8872 = t4601 * t2298;
    let t8881 = t2604 * t2301;
    let t8884 = t645 * t1614;
    let t8885 = t903 * t8884;
    let t8889 = t7844 * t8642;
    (t8854, t8856, t8858, t8860, t8862, t8864, t8872, t8881, t8884, t8885, t8889)
}
