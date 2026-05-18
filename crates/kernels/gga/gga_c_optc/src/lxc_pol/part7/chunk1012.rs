//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1012/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1012<F: Float>(t1986: F, t6636: F, t6642: F, t539: F, t6828: F, t544: F, t1846: F, t1863: F, t22120: F, t601: F, t6427: F, t2040: F, t8: F) -> (F, F, F, F, F, F, F) {
    let t22133 = t1986 * t6636;
    let t22134 = F::new(0.41015588084031179722e4) * t22133;
    let t22135 = t1986 * t6642;
    let t22136 = F::new(0.23392893589820816284e1) * t22135;
    let t22140 = t539 * t6828;
    let t22141 = F::new(16.0) * t22140;
    let t22142 = t544 * t6828;
    let t22143 = F::new(16.0) * t22142;
    let t22148 = F::new(1.0) / t1863 / t1846;
    let t22152 = F::new(0.12304676425209353917e5) * t601 * t22148 * t22120 * t6427;
    let t22154 = F::new(1.0) / t8 / t2040;
    (t22134, t22136, t22141, t22143, t22148, t22152, t22154)
}
