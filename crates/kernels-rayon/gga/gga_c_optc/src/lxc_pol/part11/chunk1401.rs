//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1401/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1401(t59022: f64, t8915: f64, t1: f64, t11940: f64, t12026: f64, t12621: f64, t15240: f64, t15690: f64, t17662: f64, t17689: f64, t17729: f64, t17908: f64, t26141: f64, t27174: f64, t27209: f64, t27215: f64, t3107: f64, t3116: f64, t3119: f64, t4310: f64, t438: f64, t450: f64, t4570: f64, t46590: f64, t47155: f64, t5290: f64, t5311: f64, t5319: f64, t5325: f64, t54245: f64, t54248: f64, t54261: f64, t54268: f64, t54295: f64, t54298: f64, t59004: f64) -> (f64, f64) {
    let t59030 = t59022 * t8915;
    let t59075 = -0.84444057528615922988e5_f64 * t27209 * t450 * t59030 * t1 + 0.3283935570557285894e5_f64 * t27215 * t450 * t59022 * t3107 * t1 - 0.23456682646837756387e4_f64 * t27174 * t450 * t59022 * t1 * t438 - t46590 / 216.0_f64 + 0.18933502127510156893e0_f64 * t54245 + t54248 / 36.0_f64 - 0.39071054849961942054e3_f64 * t54261 + 0.28345352648723563784e5_f64 * t54268 + 0.29303291137471456541e3_f64 * t11940 * t17729 - 0.2840025319126523534e0_f64 * t3116 * t15240 * t17662 + 0.23666877659387696117e0_f64 * t3116 * t12621 * t4570 * t5311 * t3119 + 0.42074449172244793095e0_f64 * t3116 * t26141 * t59004 - 0.15146801702008125515e1_f64 * t12026 * t17689 + 0.95929744112718128262e1_f64 * t47155 * t5325 - 28.0_f64 / 243.0_f64 * t4310 * t17908 - 11.0_f64 / 27.0_f64 * t15690 * t5319 + 22.0_f64 / 81.0_f64 * t15690 * t5290 + t54295 / 216.0_f64 + 7.0_f64 / 486.0_f64 * t54298;
    (t59030, t59075)
}
