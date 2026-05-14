//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1431/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1431<F: Float>(t35430: F, t9739: F, t10009: F, t113181: F, t116888: F, t117690: F, t118142: F, t118223: F, t118237: F, t118275: F, t118419: F, t121679: F, t121683: F, t25001: F, t33208: F, t33297: F, t34395: F, t34416: F, t34424: F, t34429: F, t34435: F, t35383: F, t9743: F) -> (F,) {
    let t122922 = t35430 * t9739;
    let t122929 = 0.46296296296296296296e-2 * t113181 * t118142 * t25001 - 0.34722222222222222222e-2 * t117690 * t10009 - 0.34722222222222222222e-2 * t118419 * t10009 + 0.69444444444444444444e-2 * t34435 * t34395 + 0.34722222222222222222e-2 * t33297 * t35383 + 0.34722222222222222222e-2 * t33208 * t35383 - 0.20833333333333333334e-1 * t34416 * t34424 - 0.10416666666666666667e-1 * t34416 * t34429 - 0.40208333333333333335e-2 * t118275 * t34429 - 0.17361111111111111111e-2 * t122922 * t9743 - 0.51588271604938271603e-3 * t116888 - 0.11607361111111111111e-2 * t121679 - 0.38691203703703703703e-3 * t121683 + 0.92592592592592592593e-2 * t118223 - t118237;
    (t122929,)
}
