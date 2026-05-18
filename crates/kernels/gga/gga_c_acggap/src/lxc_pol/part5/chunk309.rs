//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 309/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk309<F: Float>(t1072: F, t19: F, t661: F, t1068: F, t136: F, t1048: F, t1050: F, t1054: F, t1057: F, t1063: F, t1066: F) -> (F, F, F, F) {
    let t1074 = t1072 * t19 * t661;
    let t1075 = t1068 * t136 * t1074;
    let t1076 = t1075 / F::new(12.0);
    let t1077 = t1048 + F::new(2.0) / F::new(3.0) * t1050 - t1054 + t1057 / F::new(2.0) - t1063 / F::new(12.0) - t1066 / F::new(4.0) + t1076;
    (t1074, t1075, t1076, t1077)
}
