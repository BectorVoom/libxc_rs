//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1197/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1197<F: Float>(t22430: F, t28624: F, t22381: F, t27544: F, t22348: F, t5916: F, t97801: F, t22338: F, t28629: F, t28594: F, t5932: F, t28640: F, t5910: F, t1468: F, t22427: F, t27517: F, t29470: F) -> (F, F, F, F, F, F, F, F, F) {
    let t102982 = t28624 * t22430;
    let t102985 = t27544 * t22381;
    let t102987 = t28624 * t22348;
    let t102989 = t97801 * t5916;
    let t102991 = t28629 * t22338;
    let t102993 = t28594 * t5932;
    let t102995 = t28640 * t5910;
    let t102997 = t1468 * t22427;
    let t102999 = t27517 * t29470;
    (t102982, t102985, t102987, t102989, t102991, t102993, t102995, t102997, t102999)
}
