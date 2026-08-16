//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1006/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1006(t188: f64, t1912: f64, t193: f64, t195: f64, t1956: f64, t197: f64, t201: f64, t21868: f64, t21962: f64, t21964: f64, t21968: f64, t21970: f64, t21973: f64, t21975: f64, t21977: f64, t21981: f64, t21988: f64, t21991: f64, t21995: f64, t21998: f64, t22001: f64, t22004: f64, t22052: f64, t2238: f64, t3575: f64, t5: f64, t6668: f64, t743: f64, t750: f64) -> f64 {
    let t22063 = 3.0_f64 * t21962 + 6.0_f64 * t21964 + 3.0_f64 * t2238 * t1912 + 140.0_f64 / 3.0_f64 * t21968 + 140.0_f64 / 3.0_f64 * t21970 + 70.0_f64 / 3.0_f64 * t21973 - 14.0_f64 * t21975 - 28.0_f64 * t21977 - 1820.0_f64 / 27.0_f64 * t21981 + 3.0_f64 * t2238 * t1956 + t188 * t743 * t5 * (-t21988 + 123200.0_f64 / 243.0_f64 * t21991 - 4400.0_f64 / 27.0_f64 * t21995 - 4400.0_f64 / 27.0_f64 * t21998 + 800.0_f64 / 9.0_f64 * t22001 + 800.0_f64 / 27.0_f64 * t22004 - 25.0_f64 / 3.0_f64 * t193 * t195 * t21868 * t197 - 100.0_f64 / 9.0_f64 * t193 * t3575 * t6668 - 25.0_f64 / 9.0_f64 * t193 * t750 * t197 * t22052) * t201 / 2.0_f64;
    t22063
}
