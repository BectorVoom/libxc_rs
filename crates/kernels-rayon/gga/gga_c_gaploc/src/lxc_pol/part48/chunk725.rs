//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 725/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk725(t13078: f64, t13119: f64, t11849: f64, t959: f64, t11823: f64, t7785: f64, t13559: f64, t531: f64, t13525: f64, t808: f64, t568: f64, t12693: f64, t12706: f64, t13121: f64, t13143: f64, t13147: f64, t13151: f64, t797: f64, t813: f64) -> (f64, f64, f64, f64) {
    let t13697 = 0.59584149919750711116e-1_f64 * t13078;
    let t13700 = 0.11916829983950142223e0_f64 * t13119;
    let t13702 = t11849 * t959;
    let t13703 = 0.14896037479937677779e-1_f64 * t13702;
    let t13704 = t11823 * t7785;
    let t13706 = t531 * t13559;
    let t13709 = t808 * t13525;
    let t13710 = t568 * t13709;
    let t13716 = t13697 - 0.63904876589867916126e-1_f64 * t12693 + 0.63904876589867916126e-1_f64 * t12706 + t13700 + 0.59584149919750711116e-1_f64 * t13121 + t13703 - 0.44688112439813033338e-1_f64 * t13704 - 0.35750489951850426669e0_f64 * t797 * t13706 - 0.23005755572352449806e1_f64 * t813 * t13710 + 0.63904876589867916128e-1_f64 * t13143 - 0.59584149919750711116e-1_f64 * t13147 - 0.63904876589867916128e-1_f64 * t13151;
    (t13706, t13709, t13710, t13716)
}
