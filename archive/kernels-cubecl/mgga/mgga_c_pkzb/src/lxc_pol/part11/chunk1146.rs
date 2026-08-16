//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1146/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1146<F: Float>(t3650: F, t785: F, t2036: F, t25113: F, t2156: F, t9716: F, t3698: F, t6065: F, t2242: F, t9837: F, t3932: F, t6362: F) -> (F, F, F, F, F, F) {
    let t26659 = t785 * t3650;
    let t26667 = t2036 * t25113;
    let t26695 = t2036 * t26659;
    let t26780 = t9716 * t2156;
    let t26809 = t3698 * t6065;
    let t26880 = t9837 * t2242;
    let t26901 = t3932 * t6362;
    (t26667, t26695, t26780, t26809, t26880, t26901)
}
