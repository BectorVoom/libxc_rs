//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 583/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk583<F: Float>(t1037: F, t1771: F, t2: F, t9224: F, t1033: F, t8282: F, t157: F, t1985: F, t1017: F, t604: F, t12362: F, t12571: F, t526: F, t1045: F, t2101: F, t2178: F, t358: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t12809 = t1771 * t1037;
    let t12823 = t9224 * t2;
    let t12852 = t8282 * t1033;
    let t12968 = t1985 * t157;
    let t12969 = t604 * t1017;
    let t13119 = 4.0 / 27.0 * t12362;
    let t13123 = 4.0 / 9.0 * t12571;
    let t13140 = t526 * t157;
    let t13153 = t2101 * t1045;
    let t13165 = t2178 * t358;
    (t12809, t12823, t12852, t12968, t12969, t13119, t13123, t13140, t13153, t13165)
}
