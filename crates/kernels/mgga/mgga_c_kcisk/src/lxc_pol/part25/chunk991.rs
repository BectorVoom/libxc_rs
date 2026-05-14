//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 991/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk991<F: Float>(t1725: F, t17496: F, t17402: F, t17399: F, t10934: F, t10937: F, t10941: F, t10944: F, t10947: F, t17379: F, t17382: F, t17405: F, t17408: F, t17412: F, t17417: F, t17420: F, t17435: F, t17458: F, t17463: F) -> (F, F) {
    let t17497 = t17496 * t1725;
    let t17505 = 0.41203703703703703704e-2 * t17402;
    let t17506 = 0.12361111111111111111e-1 * t17399;
    let t17516 = -t10934 - 0.82407407407407407407e-2 * t10937 + 0.20601851851851851852e-2 * t10941 - 0.61805555555555555556e-2 * t10944 + 0.30902777777777777778e-2 * t10947 - 0.41203703703703703704e-2 * t17382 + t17505 - t17506 - 0.67986111111111111113e-1 * t17379 - 0.10300925925925925926e-1 * t17408 + 0.37083333333333333333e-1 * t17458 + 0.24722222222222222222e-1 * t17412 - 0.61805555555555555555e-2 * t17405 - 0.55625000000000000001e-1 * t17463 - 0.74166666666666666668e-1 * t17420 + 0.18541666666666666667e-1 * t17417 + 0.18541666666666666667e-1 * t17435;
    (t17497, t17516)
}
