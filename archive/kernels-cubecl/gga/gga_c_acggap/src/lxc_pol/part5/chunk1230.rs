//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1230/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1230<F: Float>(t1745: F, t974: F, t4389: F, t6332: F, t1886: F, t3237: F, t6110: F, t997: F, t1036: F, t1037: F, t386: F, t5679: F) -> (F, F, F, F, F) {
    let t22538 = t974 * t1745;
    let t22540 = t4389 * t6332;
    let t22544 = t3237 * t1886;
    let t22546 = t997 * t6110;
    let t22550 = t1036 * t386 * t5679 * t1037;
    (t22538, t22540, t22544, t22546, t22550)
}
