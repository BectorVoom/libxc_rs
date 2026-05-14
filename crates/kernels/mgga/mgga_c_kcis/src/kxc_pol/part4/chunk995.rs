//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 995/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk995<F: Float>(t13953: F, t950: F, t931: F, t13712: F, t13714: F, t13710: F, t13717: F, t13720: F, t13723: F, t13726: F, t13729: F, t13732: F, t13735: F, t13738: F, t13742: F, t9681: F, t9683: F, t9691: F, t9700: F, t9775: F) -> (F, F) {
    let t13954 = t13953 * t950;
    let t13956 = 1.0 * t931 * t13954;
    let t13962 = 0.41203703703703703704e-2 * t13712;
    let t13963 = 0.12361111111111111111e-1 * t13714;
    let t13973 = -t9775 - 0.82407407407407407407e-2 * t9691 + 0.20601851851851851852e-2 * t9683 - 0.61805555555555555556e-2 * t9700 + 0.30902777777777777778e-2 * t9681 - 0.41203703703703703704e-2 * t13710 + t13962 - t13963 + 0.67986111111111111113e-1 * t13717 - 0.10300925925925925926e-1 * t13720 + 0.37083333333333333333e-1 * t13723 - 0.24722222222222222222e-1 * t13726 - 0.61805555555555555555e-2 * t13729 - 0.55625000000000000001e-1 * t13732 + 0.74166666666666666668e-1 * t13735 + 0.18541666666666666667e-1 * t13738 - 0.18541666666666666667e-1 * t13742;
    (t13956, t13973)
}
