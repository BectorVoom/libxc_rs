//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 868/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk868<F: Float>(t18944: F, t950: F, t931: F, t6393: F, t9655: F, t13710: F, t13717: F, t13962: F, t13963: F, t18645: F, t18650: F, t18655: F, t18659: F, t18661: F, t18664: F, t18667: F, t18669: F, t18674: F, t18679: F, t18683: F, t9691: F, t9775: F) -> (F, F, F) {
    let t18945 = t18944 * t950;
    let t18947 = F::new(1.0) * t931 * t18945;
    let t18949 = F::cast_from(0.16081824322151104822e2_f64) * t9655 * t6393;
    let t18964 = -t9775 - F::cast_from(0.41203703703703703703e-2_f64) * t9691 - F::cast_from(0.82407407407407407408e-2_f64) * t13710 + t13962 - t13963 + F::cast_from(0.12361111111111111111e-1_f64) * t13717 + F::cast_from(0.20601851851851851852e-2_f64) * t18645 - F::cast_from(0.10300925925925925926e-1_f64) * t18650 + F::cast_from(0.37083333333333333333e-1_f64) * t18655 - F::cast_from(0.24722222222222222222e-1_f64) * t18659 - F::cast_from(0.61805555555555555557e-2_f64) * t18661 - F::cast_from(0.55625000000000000001e-1_f64) * t18664 + F::cast_from(0.74166666666666666668e-1_f64) * t18667 + F::cast_from(0.30902777777777777778e-2_f64) * t18669 - F::cast_from(0.61805555555555555555e-2_f64) * t18674 + F::cast_from(0.18541666666666666667e-1_f64) * t18679 - F::cast_from(0.92708333333333333333e-2_f64) * t18683;
    (t18947, t18949, t18964)
}
