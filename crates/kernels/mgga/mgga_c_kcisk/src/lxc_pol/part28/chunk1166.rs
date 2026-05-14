//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1166/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1166<F: Float>(t34057: F, t415: F, t7070: F, t9687: F, t5074: F, t9946: F, t34045: F, t9664: F, t32891: F, t32897: F, t32901: F, t32910: F, t32925: F, t32942: F, t32990: F, t34055: F, t9922: F) -> (F, F, F, F, F, F) {
    let t34058 = t415 * t34057;
    let t34060 = t9687 * t7070;
    let t34061 = t415 * t34060;
    let t34065 = t5074 * t9946;
    let t34067 = t9664 * t34045;
    let t34070 = t32891 + 0.10416666666666666667e-1 * t32942 * t9922 + 0.10416666666666666667e-1 * t32990 * t9922 + 0.24872916666666666666e-2 * t34055 - 0.24872916666666666666e-2 * t34058 - 0.24872916666666666666e-2 * t34061 - t32897 + 0.11054629629629629629e-2 * t32901 + 0.34722222222222222223e-2 * t32910 + 0.11054629629629629629e-2 * t34065 + 0.34722222222222222223e-2 * t34067 + 0.11054629629629629629e-2 * t32925;
    (t34058, t34060, t34061, t34065, t34067, t34070)
}
