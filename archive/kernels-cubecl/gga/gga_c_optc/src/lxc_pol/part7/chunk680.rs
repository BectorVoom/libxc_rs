//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 680/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk680<F: Float>(t572: F, t6382: F, t1824: F, t564: F, t62: F, t1810: F, t571: F, t1827: F, t70: F, t67: F, t1828: F, t1863: F, t80: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t6383 = t6382 * t572;
    let t6387 = F::cast_from(1.0_f64) / t1824 / t564;
    let t6388 = t62 * t6387;
    let t6389 = t1810 * t571;
    let t6391 = F::cast_from(1.0_f64) / t1827 / t70;
    let t6392 = t6389 * t6391;
    let t6395 = t6389 * t572;
    let t6399 = F::cast_from(1.0_f64) / t1824 / t67;
    let t6400 = t62 * t6399;
    let t6401 = t6389 * t1828;
    let t6405 = F::cast_from(1.0_f64) / t1863 / t80;
    (t6383, t6387, t6388, t6391, t6392, t6395, t6399, t6400, t6401, t6405)
}
