//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 918/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk918<F: Float>(t13953: F, t950: F, t931: F, t13712: F, t13714: F, t13710: F, t13717: F, t13720: F, t13723: F, t13726: F, t13729: F, t13732: F, t13735: F, t13738: F, t13742: F, t9681: F, t9683: F, t9691: F, t9700: F, t9775: F) -> (F, F) {
    let t13954 = t13953 * t950;
    let t13956 = F::new(1.0) * t931 * t13954;
    let t13962 = F::new(0.41203703703703703704e-2) * t13712;
    let t13963 = F::new(0.12361111111111111111e-1) * t13714;
    let t13973 = -t9775 - F::new(0.82407407407407407407e-2) * t9691 + F::new(0.20601851851851851852e-2) * t9683 - F::new(0.61805555555555555556e-2) * t9700 + F::new(0.30902777777777777778e-2) * t9681 - F::new(0.41203703703703703704e-2) * t13710 + t13962 - t13963 + F::new(0.67986111111111111113e-1) * t13717 - F::new(0.10300925925925925926e-1) * t13720 + F::new(0.37083333333333333333e-1) * t13723 - F::new(0.24722222222222222222e-1) * t13726 - F::new(0.61805555555555555555e-2) * t13729 - F::new(0.55625000000000000001e-1) * t13732 + F::new(0.74166666666666666668e-1) * t13735 + F::new(0.18541666666666666667e-1) * t13738 - F::new(0.18541666666666666667e-1) * t13742;
    (t13956, t13973)
}
