//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1116/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1116<F: Float>(t1992: F, t6841: F, t7585: F, t7586: F, t7413: F, t8480: F, t8947: F, t1181: F, t2068: F, t26108: F, t604: F, t25732: F) -> (F, F, F, F) {
    let t39451 = t7585 * t7586 * t1992 * t6841;
    let t39454 = t7413 * t8480 * t8947;
    let t39458 = t2068 * t1181 * t604 * t26108;
    let t39462 = t2068 * t1181 * t604 * t25732;
    (t39451, t39454, t39458, t39462)
}
