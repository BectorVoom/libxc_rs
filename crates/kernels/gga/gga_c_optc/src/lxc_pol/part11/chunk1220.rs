//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1220/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1220<F: Float>(t38332: F, t38339: F, t1256: F, t48365: F, t2034: F, t13214: F, t4599: F, t6931: F, t13174: F, t4595: F, t48577: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t56062 = F::new(120.0) * t38332;
    let t56068 = F::cast_from(0.1038945353962551798e3_f64) * t38339;
    let t56073 = t48365 * t1256;
    let t56074 = t2034 * t56073;
    let t56077 = t13214 * t4599;
    let t56078 = t6931 * t56077;
    let t56081 = t13174 * t4595;
    let t56082 = t2034 * t56081;
    let t56102 = t13174 * t4599;
    let t56103 = t6931 * t56102;
    let t56106 = t48577 * t1256;
    let t56107 = t2034 * t56106;
    (t56062, t56068, t56073, t56074, t56077, t56078, t56081, t56082, t56102, t56103, t56106, t56107)
}
