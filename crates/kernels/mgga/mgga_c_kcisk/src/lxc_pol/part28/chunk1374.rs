//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1374/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1374<F: Float>(t34045: F, t34192: F, t654: F, t9061: F, t1799: F, t9680: F, t1333: F, t35186: F, t116866: F, t116886: F, t116888: F, t121116: F, t121662: F, t121667: F, t121671: F, t121673: F, t121679: F, t121683: F, t121685: F, t32942: F, t35136: F, t9652: F) -> (F, F, F) {
    let t121687 = t34192 * t34045;
    let t121689 = t9061 * t654;
    let t121691 = t1799 * t121689 * t9680;
    let t121693 = t1333 * t35186;
    let t121695 = 0.10416666666666666667e-1 * t32942 * t35136 - 0.33163888888888888888e-2 * t121662 + 0.10416666666666666667e-1 * t121116 * t9652 + 0.40208333333333333335e-2 * t121667 * t9652 - 0.17870370370370370371e-2 * t116866 - 0.34722222222222222223e-2 * t121671 - 0.34722222222222222223e-2 * t121673 - 0.12345679012345679013e-1 * t116886 - 0.7369753086419753086e-3 * t116888 - 0.16581944444444444444e-2 * t121679 - 0.55273148148148148147e-3 * t121683 + 0.69444444444444444447e-2 * t121685 + 0.26805555555555555557e-2 * t121687 + 0.16581944444444444444e-2 * t121691 - 0.36848765432098765431e-3 * t121693;
    (t121691, t121693, t121695)
}
