//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1374/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1374<F: Float>(t115772: F, t14612: F, t2326: F, t33782: F, t2059: F, t32464: F, t32465: F, t6587: F, t115162: F, t220: F, t2331: F, t115169: F, t115179: F, t115283: F, t115284: F, t120139: F, t25342: F, t25406: F, t25413: F, t33794: F, t33817: F, t33823: F, t33914: F, t33922: F, t33923: F, t33925: F, t33941: F, t34969: F, t9524: F, t9536: F, t9851: F) -> (F, F, F, F) {
    let t120144 = t115772 * t14612 * t2326 * t33782;
    let t120149 = t32464 * t32465 * t2059 * t6587;
    let t120154 = t115162 * t32465 * t220 * t2331;
    let t120171 = 0.10416666666666666667e-1 * t9851 * t33823 - 0.23148148148148148148e-2 * t9536 * t33922 * t33923 * t25406 + 0.34722222222222222222e-2 * t9536 * t32464 * t33914 * t25406 - t115179 - 0.46296296296296296296e-2 * t33794 * t33925 + 0.20833333333333333334e-1 * t9536 * t120139 + 0.20833333333333333334e-1 * t9536 * t120144 + 0.34722222222222222222e-2 * t9536 * t120149 + 0.69444444444444444444e-2 * t9536 * t120154 + 0.34722222222222222223e-2 * t33794 * t33817 - 0.54012345679012345679e-2 * t9536 * t115283 * t115284 * t25342 - 0.92592592592592592592e-2 * t9536 * t115169 * t33923 * t25413 + 0.34722222222222222223e-2 * t33941 * t33817 + 0.52083333333333333333e-2 * t9524 * t34969;
    (t120144, t120149, t120154, t120171)
}
