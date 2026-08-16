//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2340/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2340(t20994: f64, t2563: f64, t13251: f64, t13262: f64, t16816: f64, t16836: f64, t16845: f64, t16893: f64, t16969: f64, t20908: f64, t2623: f64, t4178: f64, t4180: f64, t4182: f64, t46875: f64, t46876: f64, t58705: f64, t58709: f64, t58723: f64, t58731: f64, t58735: f64, t67607: f64) -> f64 {
    let t67920 = t2563 * t20994;
    let t67926 = 3.0_f64 / 512.0_f64 * t16836 * t16845 + t13251 * t16969 / 128.0_f64 + t46875 + t16836 * t16893 / 512.0_f64 - 3.0_f64 / 256.0_f64 * t13262 * t4180 * t67607 * t16816 + 7.0_f64 / 1536.0_f64 * t4178 * t4180 * t67607 * t4182 - 35.0_f64 / 192.0_f64 * t58705 - 35.0_f64 / 384.0_f64 * t58709 - 119.0_f64 / 4608.0_f64 * t58723 + 7.0_f64 / 768.0_f64 * t58731 + 7.0_f64 / 144.0_f64 * t67920 + 595.0_f64 / 3456.0_f64 * t46876 - 7.0_f64 / 384.0_f64 * t58735 - t2623 * t20908 / 768.0_f64;
    t67926
}
