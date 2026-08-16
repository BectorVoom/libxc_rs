//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2335/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2335(t20887: f64, t9638: f64, t13242: f64, t13251: f64, t13254: f64, t16839: f64, t16903: f64, t16935: f64, t20972: f64, t20974: f64, t20983: f64, t20986: f64, t20988: f64, t2632: f64, t2643: f64, t2645: f64, t4119: f64, t4178: f64, t4180: f64, t58480: f64, t58482: f64, t58504: f64, t58528: f64, t67607: f64, t9627: f64, t9642: f64, t9646: f64) -> f64 {
    let t67729 = t9638 * t20887;
    let t67732 = -7.0_f64 / 192.0_f64 * t58480 + 7.0_f64 / 768.0_f64 * t58482 + t13251 * t16903 / 256.0_f64 - t4178 * t2645 * t67607 * t9627 / 128.0_f64 + 3.0_f64 / 512.0_f64 * t4178 * t4180 * t16839 * t16935 - 5.0_f64 / 256.0_f64 * t9642 * t20974 - 5.0_f64 / 256.0_f64 * t2643 * t9646 * t13242 * t20972 - t13254 * t20983 / 128.0_f64 - t4178 * t2645 * t16839 * t2632 * t4119 / 128.0_f64 + t13254 * t20988 / 512.0_f64 + t4178 * t4180 * t13242 * t20986 / 512.0_f64 - 7.0_f64 / 192.0_f64 * t58504 - 7.0_f64 / 384.0_f64 * t67729 + 7.0_f64 / 48.0_f64 * t58528;
    t67732
}
