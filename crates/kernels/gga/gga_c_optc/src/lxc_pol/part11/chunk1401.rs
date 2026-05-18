//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1401/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1401<F: Float>(t59022: F, t8915: F, t1: F, t11940: F, t12026: F, t12621: F, t15240: F, t15690: F, t17662: F, t17689: F, t17729: F, t17908: F, t26141: F, t27174: F, t27209: F, t27215: F, t3107: F, t3116: F, t3119: F, t4310: F, t438: F, t450: F, t4570: F, t46590: F, t47155: F, t5290: F, t5311: F, t5319: F, t5325: F, t54245: F, t54248: F, t54261: F, t54268: F, t54295: F, t54298: F, t59004: F) -> (F, F) {
    let t59030 = t59022 * t8915;
    let t59075 = -F::new(0.84444057528615922988e5) * t27209 * t450 * t59030 * t1 + F::new(0.3283935570557285894e5) * t27215 * t450 * t59022 * t3107 * t1 - F::new(0.23456682646837756387e4) * t27174 * t450 * t59022 * t1 * t438 - t46590 / F::new(216.0) + F::new(0.18933502127510156893e0) * t54245 + t54248 / F::new(36.0) - F::new(0.39071054849961942054e3) * t54261 + F::new(0.28345352648723563784e5) * t54268 + F::new(0.29303291137471456541e3) * t11940 * t17729 - F::new(0.2840025319126523534e0) * t3116 * t15240 * t17662 + F::new(0.23666877659387696117e0) * t3116 * t12621 * t4570 * t5311 * t3119 + F::new(0.42074449172244793095e0) * t3116 * t26141 * t59004 - F::new(0.15146801702008125515e1) * t12026 * t17689 + F::new(0.95929744112718128262e1) * t47155 * t5325 - F::new(28.0) / F::new(243.0) * t4310 * t17908 - F::new(11.0) / F::new(27.0) * t15690 * t5319 + F::new(22.0) / F::new(81.0) * t15690 * t5290 + t54295 / F::new(216.0) + F::new(7.0) / F::new(486.0) * t54298;
    (t59030, t59075)
}
