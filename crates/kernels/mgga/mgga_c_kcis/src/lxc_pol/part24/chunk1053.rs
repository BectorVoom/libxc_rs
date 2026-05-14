//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1053/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1053<F: Float>(t11061: F, t7788: F, t7790: F, t1071: F, t3622: F, t26954: F, t27076: F, t26996: F, t993: F, t1095: F, t982: F, t7720: F, t9562: F, t3489: F, t34690: F, t421: F) -> (F, F, F, F, F, F, F, F) {
    let t92600 = t7788 * t11061 * t7790;
    let t92651 = t3622 * t1071;
    let t92657 = t27076 * t26954;
    let t92693 = t993 * t26996;
    let t92701 = t1095 * t982;
    let t92730 = t9562 * t7720;
    let t92732 = t27076 * t3489;
    let t92735 = t421 * t34690;
    (t92600, t92651, t92657, t92693, t92701, t92730, t92732, t92735)
}
