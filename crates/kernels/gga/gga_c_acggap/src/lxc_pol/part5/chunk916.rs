//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 916/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk916<F: Float>(t1487: F, t368: F, t384: F, t398: F, t879: F, t3101: F, t506: F, t13039: F, t527: F, t1140: F, t5188: F, t12747: F, t1470: F, t4840: F, t1165: F, t3809: F, t4282: F, t530: F) -> (F, F, F, F, F, F, F) {
    let t16921 = t384 * t398 * t368 * t1487 * t879;
    let t16926 = t384 * t398 * t368 * t506 * t3101;
    let t16928 = t13039 * t527;
    let t16930 = t1140 * t5188;
    let t16940 = t12747 * t1470;
    let t16942 = t1140 * t4840;
    let t16946 = t4282 * t1165 * t530 * t3809;
    (t16921, t16926, t16928, t16930, t16940, t16942, t16946)
}
