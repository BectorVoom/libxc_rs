//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2769/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2769(t41115: f64, t5593: f64, t13258: f64, t16932: f64, t16937: f64, t10007: f64, t13080: f64, t13176: f64, t13244: f64, t13248: f64, t13251: f64, t13254: f64, t13262: f64, t13322: f64, t16836: f64, t16839: f64, t16841: f64, t16845: f64, t16907: f64, t16914: f64, t2643: f64, t2645: f64, t40951: f64, t41123: f64, t4177: f64, t4178: f64, t4180: f64, t4181: f64, t4184: f64, t46546: f64, t46737: f64, t58289: f64, t9632: f64, t9642: f64) -> f64 {
    let t58421 = t41115 * t5593;
    let t58425 = t13258 * t16932;
    let t58427 = t13258 * t16937;
    let t58439 = 455.0_f64 / 324.0_f64 * t46546 + t9642 * t16907 / 384.0_f64 + t2643 * t2645 * t16839 * t10007 / 768.0_f64 + t9642 * t16914 / 192.0_f64 + t13176 * t4177 * t4184 / 384.0_f64 + t16836 * t13244 / 384.0_f64 + t16836 * t13248 / 768.0_f64 - t46737 * t16841 / 256.0_f64 - t13262 * t4180 * t16839 * t40951 / 512.0_f64 + t13254 * t16845 / 256.0_f64 + t4178 * t4180 * t16839 * t9632 / 512.0_f64 + 119.0_f64 / 1728.0_f64 * t58421 - 5.0_f64 / 384.0_f64 * t13251 * t13080 + 7.0_f64 / 288.0_f64 * t58425 - 7.0_f64 / 576.0_f64 * t58427 - t4178 * t2645 * t16839 * t41123 / 384.0_f64 + t4178 * t4180 * t4181 * t58289 / 768.0_f64 + t13251 * t13322 / 192.0_f64;
    t58439
}
