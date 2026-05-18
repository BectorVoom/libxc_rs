//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1280/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1280<F: Float>(t1036: F, t1459: F, t1782: F, t398: F, t864: F, t1032: F, t5569: F, t1841: F, t3765: F, t1163: F, t1165: F, t5127: F, t5922: F) -> (F, F, F, F) {
    let t23676 = t1036 * t398 * t1459 * t1782 * t864;
    let t23680 = t1032 * t5569;
    let t23682 = t3765 * t1841;
    let t23686 = t1163 * t1165 * t5922 * t5127;
    (t23676, t23680, t23682, t23686)
}
