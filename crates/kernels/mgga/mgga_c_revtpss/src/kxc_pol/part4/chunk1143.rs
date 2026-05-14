//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1143/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1143<F: Float>(t1663: F, t371: F, t676: F, t1025: F, t11922: F, t4901: F, t4899: F, t1028: F, t11779: F, t11792: F, t11994: F, t15724: F, t15725: F, t15728: F, t15732: F, t15736: F, t15744: F, t15745: F, t1665: F, t4839: F, t4875: F) -> (F,) {
    let t15749 = t371 * t676 * t1663;
    let t15750 = t1025 * t15749;
    let t15752 = t11922 * t4901;
    let t15754 = 0.28582678745379824648e-3 * t4899 * t15752;
    let t15755 = t15724 + 0.85748036236139473944e-3 * t15725 * t4839 - 0.45732285992607719436e-2 * t15728 * t4839 - 0.47637797908966374413e-4 * t15732 - t15736 - 0.28582678745379824648e-3 * t11994 * t4875 + 0.22866142996303859718e-2 * t11792 * t1665 - 0.72409452821628889107e-2 * t11779 * t1665 + t15744 + 0.22866142996303859718e-2 * t15745 * t1028 + 0.47637797908966374413e-4 * t15750 - t15754;
    (t15755,)
}
