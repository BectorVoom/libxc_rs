//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1062/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1062<F: Float>(t12812: F, t3270: F, t3269: F, t3275: F, t3472: F, t42424: F, t11189: F, t42419: F, t43798: F, t11506: F, t41344: F, t12033: F, t40276: F, t42966: F, t3579: F, t41348: F) -> (F, F, F, F, F, F, F, F) {
    let t44078 = t3270 * t12812;
    let t44080 = t3269 * t44078 / 4.0;
    let t44083 = 5.0 / 16.0 * t3275 * t3472 * t42424;
    let t44086 = 45.0 / 64.0 * t3275 * t11189 * t42419;
    let t44089 = 5.0 / 8.0 * t3275 * t3472 * t43798;
    let t44091 = 3.0 / 2.0 * t11506 * t41344;
    let t44093 = t40276 * t12033 / 2.0;
    let t44096 = 5.0 / 8.0 * t3275 * t3472 * t42966;
    let t44098 = t3579 * t41348 / 2.0;
    (t44080, t44083, t44086, t44089, t44091, t44093, t44096, t44098)
}
