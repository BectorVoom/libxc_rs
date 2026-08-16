//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1695/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1695<F: Float>(t12199: F, t3741: F, t3732: F, t792: F, t118: F, t3734: F, t794: F, t3719: F, t3739: F, t782: F) -> (F, F, F, F, F, F, F) {
    let t12200 = t12199 * t3741;
    let t12202 = t792 * t3732;
    let t12204 = t118 * t794 * t3734;
    let t12205 = t12202 * t12204;
    let t12208 = t118 * t794 * t3719;
    let t12209 = t3739 * t12208;
    let t12211 = t782 * t3732;
    (t12200, t12202, t12204, t12205, t12208, t12209, t12211)
}
