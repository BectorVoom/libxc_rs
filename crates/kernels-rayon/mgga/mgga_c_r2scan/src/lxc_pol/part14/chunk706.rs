//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 706/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk706(t1894: f64, t5447: f64, t5448: f64, t1647: f64, t1898: f64, t1907: f64, t1945: f64, t61: f64, t1719: f64, t695: f64, t721: f64, t1981: f64) -> (f64, f64, f64, f64, f64) {
    let t5451 = 0.62071215503128080361e4_f64 * t5447 * t1894 * t5448;
    let t5454 = 0.28947563097646563121e3_f64 * t1907 * t1898 * t1647;
    let t5455 = t61 * t1945;
    let t5456 = t1719 * t695;
    let t5457 = t721 * t5456;
    let t5459 = 0.31168546390226634765e3_f64 * t5455 * t5457;
    let t5460 = t61 * t1981;
    (t5451, t5454, t5456, t5459, t5460)
}
