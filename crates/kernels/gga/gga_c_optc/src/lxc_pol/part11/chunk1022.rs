//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1022/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1022<F: Float>(t16572: F, t1975: F, t16606: F, t740: F, t16301: F, t732: F, t13053: F, t1310: F, t12997: F, t13004: F, t1320: F, t3546: F, t4759: F, t108: F, t16287: F, t176: F, t203: F, t729: F) -> (F, F, F, F, F, F, F, F) {
    let t48013 = t16572 * t1975;
    let t48017 = t16606 * t740;
    let t48024 = t732 * t16301;
    let t48028 = t1310 * t13053;
    let t48040 = t1310 * t12997;
    let t48045 = t13004 * t1320;
    let t48051 = t3546 * t4759;
    let t48058 = t176 * t729 * t16287 * t108 * t203;
    (t48013, t48017, t48024, t48028, t48040, t48045, t48051, t48058)
}
