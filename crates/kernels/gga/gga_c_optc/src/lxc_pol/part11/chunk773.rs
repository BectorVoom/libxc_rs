//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 773/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk773<F: Float>(t2849: F, t438: F, t1135: F, t5328: F, t19: F, t2586: F, t5301: F, t1133: F, t4369: F, t4380: F, t309: F, t5279: F, t441: F) -> (F, F, F, F, F, F, F) {
    let t15305 = t438 * t2849;
    let t15310 = t1135 * t5328;
    let t15311 = t15310 * t19;
    let t15321 = t2586 * t5301;
    let t15322 = t1133 * t15321;
    let t15324 = t4369 * t4380;
    let t15326 = t5279 * t309;
    let t15327 = t441 * t15326;
    (t15305, t15311, t15321, t15322, t15324, t15326, t15327)
}
