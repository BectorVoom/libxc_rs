//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 938/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk938<F: Float>(t1165: F, t12731: F, t1532: F, t17775: F, t14047: F, t4273: F, t1106: F, t1181: F, t1567: F, t3361: F, t1140: F, t4791: F, t3409: F, t4300: F, t12935: F, t3355: F, t3402: F, t530: F) -> (F, F, F, F, F, F) {
    let t17778 = t12731 * t1165 * t1532 * t17775;
    let t17798 = t14047 * t4273;
    let t17804 = t3361 * t1181 * t1567 * t1106;
    let t17811 = t1140 * t4791;
    let t17821 = t3409 * t4300;
    let t17826 = t12935 * t3402 * t1165 * t530 * t3355;
    (t17778, t17798, t17804, t17811, t17821, t17826)
}
