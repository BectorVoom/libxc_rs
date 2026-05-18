//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1230/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1230<F: Float>(t27006: F, t27014: F, t10995: F, t7787: F, t26954: F, t27076: F, t26996: F, t993: F, t1095: F, t982: F, t11081: F, t26960: F, t26962: F) -> (F, F, F, F, F, F) {
    let t92607 = t27014 * t27006;
    let t92613 = t7787 * t10995;
    let t92657 = t27076 * t26954;
    let t92693 = t993 * t26996;
    let t92701 = t1095 * t982;
    let t92718 = t26960 * t11081 * t26962;
    (t92607, t92613, t92657, t92693, t92701, t92718)
}
