//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1013/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1013<F: Float>(t2506: F, t31014: F, t1434: F, t193: F, t1424: F, t5120: F, t743: F, t6109: F, t1154: F, t6837: F, t5053: F) -> (F, F, F, F, F, F, F, F, F) {
    let t31015 = t2506 * t31014;
    let t31017 = t1434 * t193 * t31015;
    let t31019 = t1424 * t5120;
    let t31020 = t743 * t31019;
    let t31022 = t6109 * t193 * t31020;
    let t31024 = t6837 * t1154;
    let t31025 = t743 * t31024;
    let t31027 = t6109 * t193 * t31025;
    let t31029 = t1424 * t5053;
    (t31015, t31017, t31019, t31020, t31022, t31024, t31025, t31027, t31029)
}
