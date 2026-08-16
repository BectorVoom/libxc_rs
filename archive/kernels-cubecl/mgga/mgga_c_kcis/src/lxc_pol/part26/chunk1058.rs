//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1058/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1058<F: Float>(t26602: F, t7592: F, t7583: F, t7579: F, t9229: F, t209: F, t2415: F, t7581: F) -> (F, F, F, F, F) {
    let t26603 = t26602 * t7592;
    let t26605 = t26602 * t7583;
    let t26607 = t9229 * t7579;
    let t26608 = t26607 * t7583;
    let t26611 = t209 * t7581 * t2415;
    (t26603, t26605, t26607, t26608, t26611)
}
