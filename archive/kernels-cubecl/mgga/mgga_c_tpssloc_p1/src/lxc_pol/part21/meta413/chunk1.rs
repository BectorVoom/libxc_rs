//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1926/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1926<F: Float>(t3287: F, t4756: F, t1102: F, t3279: F, t4764: F, t4772: F, t699: F) -> (F, F, F) {
    let t14813 = t3287 * t4756;
    let t14814 = t14813 * t1102;
    let t14816 = t4764 * t3279;
    let t14818 = t699 * t4772;
    (t14814, t14816, t14818)
}
