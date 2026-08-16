//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 973/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk973(t1022: f64, t9755: f64, t2639: f64, t787: f64, t28002: f64, t9858: f64, t13141: f64, t2464: f64, t2684: f64, t13097: f64, t13098: f64, t13101: f64, t13102: f64, t13105: f64, t13106: f64, t13109: f64, t13110: f64, t1589: f64, t1628: f64, t2049: f64, t2194: f64, t2197: f64, t313: f64, t314: f64, t317: f64, t43081: f64, t43082: f64, t43527: f64, t43529: f64, t43567: f64, t43569: f64, t43571: f64, t531: f64, t568: f64, t769: f64, t784: f64, t797: f64, t808: f64, t813: f64, t833: f64, t836: f64) -> (f64, f64) {
    let t43572 = t9755 * t1022;
    let t43575 = 0.53625734927775640005e1_f64 * t787 * t43572 * t2639;
    let t43579 = 0.17875244975925213335e2_f64 * t787 * t28002 * t1022 * t9858;
    let t43581 = t2684 * t2464 * t13141;
    let t43582 = 0.63904876589867916128e-1_f64 * t43581;
    let t43583 = t43527 - 0.29792074959875355558e-1_f64 * t43529 - 0.35750489951850426669e0_f64 * t2049 * t13102 - 0.35750489951850426669e0_f64 * t797 * t531 * t43082 + 0.23005755572352449806e1_f64 * t833 * t568 * t836 * t43081 + 0.23005755572352449806e1_f64 * t2197 * t13110 + 0.23833659967900284446e0_f64 * t13098 * t784 + 0.35750489951850426669e0_f64 * t313 * t314 * t43081 * t317 - 0.23005755572352449806e1_f64 * t2194 * t13106 - 0.23005755572352449806e1_f64 * t813 * t568 * t808 * t43081 - 0.30674340763136599741e1_f64 * t813 * t1628 * t13105 - 0.23833659967900284446e0_f64 * t797 * t1589 * t13101 + 0.30674340763136599741e1_f64 * t833 * t1628 * t13109 + 0.35750489951850426669e0_f64 * t769 * t13097 * t317 + t43567 + t43569 + t43571 - t43575 + t43579 - t43582;
    (t43572, t43583)
}
