//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 867/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk867<F: Float>(t1240: F, t3101: F, t381: F, t1032: F, t3811: F, t151: F, t3668: F, t940: F, t947: F, t3765: F, t932: F, t1077: F, t435: F) -> (F, F, F, F, F, F) {
    let t12419 = t381 * t1240 * t3101;
    let t12421 = t1032 * t3811;
    let t12457 = t151 * t940 * t3668;
    let t12458 = t12457 * t947;
    let t12460 = t3765 * t932;
    let t12473 = t435 * t1077;
    (t12419, t12421, t12457, t12458, t12460, t12473)
}
