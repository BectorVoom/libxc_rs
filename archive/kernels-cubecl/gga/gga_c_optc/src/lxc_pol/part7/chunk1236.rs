//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1236/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1236<F: Float>(t1659: F, t25514: F, t2367: F, t8036: F, t930: F, t25052: F, t953: F, t2765: F, t7878: F, t940: F, t2708: F, t8257: F) -> (F, F, F, F, F) {
    let t25518 = t1659 * t25514;
    let t25522 = t930 * t2367 * t8036;
    let t25524 = t953 * t25052;
    let t25529 = t940 * t7878 * t2765;
    let t25531 = t2708 * t8257;
    (t25518, t25522, t25524, t25529, t25531)
}
