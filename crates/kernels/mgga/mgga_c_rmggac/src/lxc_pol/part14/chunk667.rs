//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 667/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk667<F: Float>(t884: F, t8866: F, t2405: F, t302: F, t72: F, t2298: F, t4601: F, t2025: F, t5928: F, t1664: F, t668: F, t289: F) -> (F, F, F, F, F, F, F) {
    let t8867 = t884 * t8866;
    let t8869 = t302 * t2405;
    let t8870 = t72 * t8869;
    let t8872 = t4601 * t2298;
    let t8874 = t5928 * t2025;
    let t8876 = t1664 * t668;
    let t8877 = t289 * t8876;
    (t8867, t8869, t8870, t8872, t8874, t8876, t8877)
}
