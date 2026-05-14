//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1364/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1364<F: Float>(t119714: F, t119716: F, t119718: F, t119721: F, t119723: F, t119725: F, t119727: F, t119729: F, t119731: F, t119733: F, t119735: F, t119738: F, t119740: F, t119742: F, t119744: F, t119746: F, t119748: F, t119750: F) -> (F,) {
    let t119926 = 0.17986111111111111111e-1 * t119714 - 0.25e0 * t119716 - 0.1875e0 * t119718 - 0.9375e-1 * t119721 + 0.26979166666666666667e-1 * t119723 + 0.26979166666666666667e-1 * t119725 + 0.25e0 * t119727 - 0.41666666666666666667e-1 * t119729 - 0.89930555555555555557e-2 * t119731 - 0.1875e0 * t119733 + 0.625e-1 * t119735 + 0.625e-1 * t119738 + 0.20234375e-1 * t119740 - 0.53958333333333333333e-1 * t119742 + 0.53958333333333333334e-1 * t119744 + 0.53958333333333333334e-1 * t119746 + 0.89930555555555555557e-2 * t119748 - 0.4046875e-1 * t119750;
    (t119926,)
}
