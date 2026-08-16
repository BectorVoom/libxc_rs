//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2336/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2336(t20969: f64, t2639: f64, t16752: f64, t2632: f64, t120: f64, t13222: f64, t13228: f64, t13251: f64, t13262: f64, t13350: f64, t13351: f64, t1512: f64, t16836: f64, t16839: f64, t16918: f64, t16932: f64, t16937: f64, t17017: f64, t20756: f64, t20986: f64, t2643: f64, t2645: f64, t41453: f64, t41467: f64, t4178: f64, t4180: f64, t4181: f64, t4255: f64, t46574: f64, t5612: f64, t58557: f64, t58765: f64, t67578: f64, t67607: f64, t829: f64) -> (f64, f64) {
    let t67735 = t2639 * t20969;
    let t67739 = t2632 * t16752;
    let t67777 = 7.0_f64 / 4608.0_f64 * t67735 - t58765 * t1512 / 1024.0_f64 + t4178 * t4180 * t4181 * t67739 / 512.0_f64 + t13251 * t16918 / 256.0_f64 - t13251 * t17017 / 1024.0_f64 - t16836 * t16932 / 128.0_f64 + t16836 * t16937 / 256.0_f64 + t13262 * t2645 * t67607 * t41453 / 128.0_f64 - 3.0_f64 / 512.0_f64 * t13262 * t4180 * t16839 * t67578 + 5.0_f64 / 128.0_f64 * t2643 * t41467 * t120 * t20756 * t829 + 5.0_f64 / 128.0_f64 * t4178 * t13350 * t13228 * t58557 - t4178 * t13222 * t20986 * t13351 / 128.0_f64 - 5.0_f64 / 256.0_f64 * t2643 * t13350 * t5612 * t4255 - t46574;
    (t67739, t67777)
}
