//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1367/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1367(t3156: f64, t7878: f64, t1133: f64, t2586: f64, t8952: f64, t2849: f64, t381: f64, t26336: f64, t22035: f64, t894: f64, t1: f64, t1111: f64, t11596: f64, t26276: f64, t26287: f64, t26317: f64, t26322: f64, t27148: f64, t27153: f64, t27158: f64, t27167: f64, t27174: f64, t27175: f64, t3245: f64, t4289: f64, t438: f64, t450: f64, t8966: f64, t8968: f64, t8973: f64) -> (f64, f64, f64, f64) {
    let t27181 = t7878 * t3156;
    let t27182 = t1133 * t27181;
    let t27184 = t2586 * t8952;
    let t27185 = t1133 * t27184;
    let t27188 = 1.0_f64 / t381 / t2849;
    let t27189 = t27188 * t26336;
    let t27191 = t894 * t27189 * t22035;
    let t27194 = -t1111 * t4289 * t26287 / 6.0_f64 + 0.36629113921839320675e2_f64 * t8966 * t8968 * t27148 - 0.73258227843678641351e2_f64 * t8973 * t8968 * t27153 + t27158 / 54.0_f64 - t1111 * t3245 * t26317 / 36.0_f64 + t1111 * t4289 * t26322 / 54.0_f64 - t27167 / 36.0_f64 + 7.0_f64 / 108.0_f64 * t1111 * t11596 * t26276 - 0.23456682646837756387e4_f64 * t27174 * t450 * t27175 * t1 * t438 - 0.24147670804968771818e-2_f64 * t27182 + 0.21464596271083352727e-1_f64 * t27185 + 0.2951381987273961e-1_f64 * t1133 * t27191;
    (t27181, t27184, t27191, t27194)
}
