//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2272/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2272(t13005: f64, t13184: f64, t13196: f64, t13203: f64, t13222: f64, t13242: f64, t13350: f64, t210: f64, t221: f64, t2571: f64, t2643: f64, t2645: f64, t2649: f64, t41014: f64, t41116: f64, t4178: f64, t4180: f64, t4181: f64, t4182: f64, t4248: f64, t46644: f64, t46839: f64, t47027: f64, t47037: f64, t47039: f64, t47044: f64, t47047: f64, t47049: f64, t776: f64, t829: f64, t9632: f64, t9981: f64) -> f64 {
    let t47071 = 7.0_f64 / 192.0_f64 * t47027 + t4178 * t4180 * t4181 * t41014 / 1536.0_f64 + t4178 * t4180 * t13242 * t9632 / 512.0_f64 + 35.0_f64 / 192.0_f64 * t47037 + 15.0_f64 / 128.0_f64 * t2643 * t47039 * t13184 * t829 + t47044 * t2649 / 128.0_f64 - 595.0_f64 / 10368.0_f64 * t47047 - 7.0_f64 / 8.0_f64 * t47049 + 3.0_f64 / 16.0_f64 * t2571 * t210 * t13203 * t776 - t4178 * t13222 * t46644 * t4182 / 128.0_f64 - 3.0_f64 / 4.0_f64 * t13005 * t221 * t46839 - 5.0_f64 / 256.0_f64 * t2643 * t13350 * t13196 * t829 - t4178 * t2645 * t4248 * t9981 / 128.0_f64 + 119.0_f64 / 576.0_f64 * t41116;
    t47071
}
