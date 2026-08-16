//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1152/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1152<F: Float>(t1181: F, t5572: F, t7351: F, t7575: F, t2016: F, t9618: F, t1488: F, t2030: F, t2313: F, t2001: F, t5551: F, t1856: F, t7605: F) -> (F, F, F, F, F) {
    let t39923 = t7575 * t1181 * t7351 * t5572;
    let t39925 = t2016 * t9618;
    let t39928 = t2030 * t1488 * t2313;
    let t39930 = t2001 * t5551;
    let t39932 = t7605 * t1856;
    (t39923, t39925, t39928, t39930, t39932)
}
