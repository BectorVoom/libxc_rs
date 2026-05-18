//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1008/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1008<F: Float>(t22081: F, t2193: F, t616: F, t1983: F, t6817: F, t1986: F, t6821: F, t1867: F, t6419: F, t601: F, t6820: F, t1859: F, t6427: F) -> (F, F, F, F, F, F, F) {
    let t22082 = F::new(4.0) * t22081;
    let t22086 = t616 * t2193;
    let t22090 = t6817 * t1983;
    let t22091 = F::new(0.14649244029402527953e-2) * t22090;
    let t22092 = t1986 * t6821;
    let t22093 = F::new(0.2077890707925103596e3) * t22092;
    let t22095 = t1867 * t6419;
    let t22098 = F::new(0.69263023597503453196e2) * t601 * t6820 * t22095;
    let t22100 = t6427 * t1859;
    (t22082, t22086, t22091, t22093, t22095, t22098, t22100)
}
