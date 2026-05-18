//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1027/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1027<F: Float>(t5267: F, t997: F, t1441: F, t3670: F, t14053: F, t1541: F, t12727: F, t1466: F, t3382: F, t4681: F, t3706: F, t524: F) -> (F, F, F, F, F, F) {
    let t17521 = t997 * t5267;
    let t17528 = t3670 * t1441;
    let t17530 = t14053 * t1541;
    let t17540 = t12727 * t1466;
    let t17542 = t3382 * t4681;
    let t17544 = t3706 * t524;
    (t17521, t17528, t17530, t17540, t17542, t17544)
}
