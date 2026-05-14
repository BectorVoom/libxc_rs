//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1045/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1045<F: Float>(t3379: F, t5608: F, t1487: F, t944: F, t3431: F, t6271: F, t1524: F, t157: F, t406: F, t1165: F, t3194: F, t4289: F, t5730: F, t1444: F, t372: F, t1449: F, t322: F) -> (F, F, F, F, F, F, F) {
    let t20961 = t3379 * t5608;
    let t20963 = t944 * t1487;
    let t20969 = t3431 * t6271;
    let t20972 = t1524 * t406 * t157;
    let t20985 = t3194 * t1165 * t4289 * t5730;
    let t20987 = t1444 * t372;
    let t20992 = t1449 * t322;
    (t20961, t20963, t20969, t20972, t20985, t20987, t20992)
}
