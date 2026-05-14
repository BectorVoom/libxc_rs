//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1434/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1434<F: Float>(t113124: F, t1636: F, t35400: F, t112934: F, t113069: F, t113181: F, t116970: F, t118316: F, t118324: F, t118330: F, t121724: F, t121730: F, t121733: F, t121736: F, t121739: F, t20: F, t25093: F, t2801: F, t2807: F, t35409: F, t35416: F, t654: F, t9720: F) -> (F, F) {
    let t123004 = t113124 * t35400 * t1636;
    let t123021 = -0.34822083333333333332e-2 * t121724 + 0.46429444444444444444e-2 * t121730 - 0.15476481481481481481e-2 * t121733 + 0.46429444444444444444e-2 * t121736 - 0.38691203703703703703e-2 * t121739 - 0.34722222222222222223e-2 * t113181 * t123004 - 0.44675925925925925926e-3 * t113069 - 0.23148148148148148148e-2 * t118316 - 0.116403125e-2 * t112934 * t35416 - 0.23148148148148148148e-2 * t118324 - t118330 - 0.61905925925925925925e-2 * t116970 - 0.52083333333333333333e-2 * t2801 * t25093 * t654 * t20 * t2807 + 0.27777777777777777778e-1 * t9720 * t35409 * t2807;
    (t123004, t123021)
}
