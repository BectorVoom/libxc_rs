//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2270/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2270(t13248: f64, t13258: f64, t1484: f64, t2631: f64, t4233: f64, t828: f64, t10007: f64, t13076: f64, t13222: f64, t13223: f64, t13242: f64, t13322: f64, t13326: f64, t13350: f64, t1510: f64, t232: f64, t2643: f64, t2645: f64, t2647: f64, t41063: f64, t41096: f64, t41108: f64, t4178: f64, t4181: f64, t4182: f64, t4240: f64, t46692: f64, t46693: f64, t9516: f64, t9616: f64, t9642: f64) -> (f64, f64) {
    let t46998 = t13258 * t13248;
    let t47012 = t1484 * t2631;
    let t47017 = t4233 * t828;
    let t47025 = t9642 * t13322 / 128.0_f64 + t2643 * t2645 * t13242 * t10007 / 256.0_f64 + t9642 * t13326 / 256.0_f64 + t2643 * t2645 * t4181 * t232 * t9516 / 768.0_f64 - t9642 * t13076 / 1024.0_f64 - 7.0_f64 / 768.0_f64 * t46998 - 5.0_f64 / 256.0_f64 * t2643 * t13350 * t1510 * t9616 + t2643 * t13222 * t13223 * t10007 / 256.0_f64 + 3.0_f64 / 512.0_f64 * t4178 * t46692 * t46693 * t4182 + t41096 - 5.0_f64 / 256.0_f64 * t2643 * t13350 * t47012 * t2647 + t2643 * t13222 * t47017 * t2647 / 128.0_f64 - 7.0_f64 / 192.0_f64 * t41108 - t41063 * t4240 / 1024.0_f64;
    (t47012, t47025)
}
