//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 929/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk929<F: Float>(t16438: F, t1664: F, t1645: F, t15991: F, t15993: F, t10570: F, t10572: F, t10574: F, t10576: F, t10761: F, t15989: F, t15996: F, t16001: F, t16006: F, t16011: F, t16015: F, t16019: F, t16024: F, t16028: F, t16032: F) -> (F, F) {
    let t16439 = t16438 * t1664;
    let t16441 = 1.0 * t1645 * t16439;
    let t16447 = 0.41203703703703703704e-2 * t15991;
    let t16448 = 0.12361111111111111111e-1 * t15993;
    let t16458 = -t10761 - 0.82407407407407407407e-2 * t10570 + 0.20601851851851851852e-2 * t10572 - 0.61805555555555555556e-2 * t10574 + 0.30902777777777777778e-2 * t10576 - 0.41203703703703703704e-2 * t15989 + t16447 - t16448 - 0.67986111111111111113e-1 * t15996 - 0.10300925925925925926e-1 * t16001 + 0.37083333333333333333e-1 * t16006 + 0.24722222222222222222e-1 * t16011 - 0.61805555555555555555e-2 * t16015 - 0.55625000000000000001e-1 * t16019 - 0.74166666666666666668e-1 * t16024 + 0.18541666666666666667e-1 * t16028 + 0.18541666666666666667e-1 * t16032;
    (t16441, t16458)
}
