//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 973/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk973<F: Float>(t1022: F, t9755: F, t2639: F, t787: F, t28002: F, t9858: F, t13141: F, t2464: F, t2684: F, t13097: F, t13098: F, t13101: F, t13102: F, t13105: F, t13106: F, t13109: F, t13110: F, t1589: F, t1628: F, t2049: F, t2194: F, t2197: F, t313: F, t314: F, t317: F, t43081: F, t43082: F, t43527: F, t43529: F, t43567: F, t43569: F, t43571: F, t531: F, t568: F, t769: F, t784: F, t797: F, t808: F, t813: F, t833: F, t836: F) -> (F, F) {
    let t43572 = t9755 * t1022;
    let t43575 = F::cast_from(0.53625734927775640005e1_f64) * t787 * t43572 * t2639;
    let t43579 = F::cast_from(0.17875244975925213335e2_f64) * t787 * t28002 * t1022 * t9858;
    let t43581 = t2684 * t2464 * t13141;
    let t43582 = F::cast_from(0.63904876589867916128e-1_f64) * t43581;
    let t43583 = t43527 - F::cast_from(0.29792074959875355558e-1_f64) * t43529 - F::cast_from(0.35750489951850426669e0_f64) * t2049 * t13102 - F::cast_from(0.35750489951850426669e0_f64) * t797 * t531 * t43082 + F::cast_from(0.23005755572352449806e1_f64) * t833 * t568 * t836 * t43081 + F::cast_from(0.23005755572352449806e1_f64) * t2197 * t13110 + F::cast_from(0.23833659967900284446e0_f64) * t13098 * t784 + F::cast_from(0.35750489951850426669e0_f64) * t313 * t314 * t43081 * t317 - F::cast_from(0.23005755572352449806e1_f64) * t2194 * t13106 - F::cast_from(0.23005755572352449806e1_f64) * t813 * t568 * t808 * t43081 - F::cast_from(0.30674340763136599741e1_f64) * t813 * t1628 * t13105 - F::cast_from(0.23833659967900284446e0_f64) * t797 * t1589 * t13101 + F::cast_from(0.30674340763136599741e1_f64) * t833 * t1628 * t13109 + F::cast_from(0.35750489951850426669e0_f64) * t769 * t13097 * t317 + t43567 + t43569 + t43571 - t43575 + t43579 - t43582;
    (t43572, t43583)
}
