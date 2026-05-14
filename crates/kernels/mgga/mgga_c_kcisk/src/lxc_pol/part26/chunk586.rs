//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 586/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk586<F: Float>(t1191: F, t5748: F, t1172: F, t2092: F, t3679: F, t1190: F, t3677: F, t3573: F, t3683: F, t5668: F, t5673: F, t5678: F, t5682: F, t334: F, t2097: F, t45: F) -> (F, F, F, F, F, F, F, F) {
    let t5749 = t5748 * t1191;
    let t5751 = 1.0 * t1172 * t5749;
    let t5752 = t2092 * t3679;
    let t5753 = t5752 * t1190;
    let t5755 = 0.16081824322151104822e2 * t3677 * t5753;
    let t5761 = t3683 + 0.30902777777777777778e-2 * t3573 + 0.30902777777777777778e-2 * t5668 - 0.61805555555555555555e-2 * t5673 + 0.18541666666666666667e-1 * t5678 - 0.18541666666666666667e-1 * t5682;
    let t5762 = t5761 * t334;
    let t5765 = t45 * t2097;
    (t5749, t5751, t5752, t5753, t5755, t5761, t5762, t5765)
}
