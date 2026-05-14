//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 818/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk818<F: Float>(t10928: F, t29195: F, t10934: F, t17382: F, t23460: F, t23472: F, t23481: F, t29082: F, t29085: F, t29088: F, t29091: F, t29094: F, t29097: F, t1725: F, t2418: F, t8729: F) -> (F, F, F, F) {
    let t29231 = t29195 * t10928;
    let t29244 = -t10934 - 0.12361111111111111111e-1 * t17382 + 0.61805555555555555556e-2 * t23460 - 0.18541666666666666667e-1 * t23472 + 0.92708333333333333334e-2 * t23481 - 0.10300925925925925926e-1 * t29082 + 0.37083333333333333333e-1 * t29085 - 0.18541666666666666666e-1 * t29088 - 0.55625000000000000001e-1 * t29091 + 0.55625000000000000001e-1 * t29094 - 0.92708333333333333333e-2 * t29097;
    let t29250 = t29195 * t1725;
    let t29253 = t2418 * t8729;
    (t29231, t29244, t29250, t29253)
}
