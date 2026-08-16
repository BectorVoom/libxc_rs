//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1036/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1036<F: Float>(t1140: F, t4791: F, t3409: F, t4300: F, t1165: F, t12935: F, t3355: F, t3402: F, t530: F, t4713: F, t13084: F, t4921: F) -> (F, F, F, F, F) {
    let t17811 = t1140 * t4791;
    let t17821 = t3409 * t4300;
    let t17826 = t12935 * t3402 * t1165 * t530 * t3355;
    let t17831 = t3409 * t4713;
    let t17837 = t13084 * t4921;
    (t17811, t17821, t17826, t17831, t17837)
}
