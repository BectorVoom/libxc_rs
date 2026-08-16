//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 875/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk875(t1173: f64, t1337: f64, t459: f64, t1163: f64, t3559: f64, t1175: f64, t3579: f64, t3539: f64, t1354: f64, t1422: f64, t3593: f64, t1364: f64) -> (f64, f64, f64, f64) {
    let t13129 = t1337 * t1173 * t459;
    let t13130 = t1163 * t3559;
    let t13131 = t13129 * t13130;
    let t13134 = t3579 * t1175;
    let t13135 = t3539 * t13134;
    let t13138 = t1422 * t1354;
    let t13139 = t1163 * t3593;
    let t13140 = t13138 * t13139;
    let t13143 = t3579 * t1364;
    (t13131, t13135, t13140, t13143)
}
