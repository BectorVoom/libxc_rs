//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1415/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1415<F: Float>(t32464: F, t3583: F, t6591: F, t33922: F, t3575: F, t1310: F, t13893: F, t539: F, t12829: F, t1597: F, t109756: F, t115094: F, t115108: F, t115144: F, t115150: F, t18953: F, t19033: F, t32339: F, t32354: F, t32439: F, t32458: F, t33784: F, t33817: F, t33832: F, t33837: F, t33923: F, t33937: F, t3579: F, t9536: F, t9867: F) -> (F, F, F) {
    let t115263 = t32464 * t6591 * t3583;
    let t115267 = t33922 * t6591 * t3575;
    let t115283 = t1310 * t13893 * t539;
    let t115284 = t1597 * t12829;
    let t115301 = 0.34722222222222222222e-2 * t32354 * t33817 + 0.17361111111111111111e-2 * t9536 * t115263 + 0.23148148148148148148e-2 * t9536 * t115267 - 0.34722222222222222222e-2 * t9536 * t32458 * t9867 * t3579 + 0.67013888888888888888e-3 * t32439 * t115144 - 0.38801041666666666666e-3 * t33937 * t115150 - 0.23148148148148148148e-2 * t9536 * t33922 * t33923 * t18953 - 0.54012345679012345679e-2 * t9536 * t115283 * t115284 * t19033 - 0.10416666666666666667e-1 * t9536 * t115094 + 0.55555555555555555557e-1 * t32339 * t33832 + 0.27777777777777777778e-1 * t32339 * t33837 + 0.10722222222222222222e-1 * t109756 * t33837 + 0.32166666666666666667e-1 * t109756 * t33784 + 0.22114583333333333334e-1 * t32439 * t115108;
    (t115263, t115267, t115301)
}
