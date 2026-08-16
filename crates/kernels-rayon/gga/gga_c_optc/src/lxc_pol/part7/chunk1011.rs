//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1011/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1011(t22120: f64, t587: f64, t601: f64, t6405: f64, t2204: f64, t2229: f64, t1998: f64, t6632: f64, t1994: f64, t22098: f64, t22103: f64, t22107: f64, t22111: f64, t22115: f64, t22117: f64, t22119: f64) -> (f64, f64, f64, f64) {
    let t22124 = 0.1403573615389248977e2_f64 * t601 * t6405 * t22120 * t587;
    let t22126 = 70.0_f64 / 3.0_f64 * t2229 * t2204;
    let t22127 = t6632 * t1998;
    let t22128 = 0.35089340384731224426e1_f64 * t22127;
    let t22129 = t6632 * t1994;
    let t22130 = 0.1038945353962551798e3_f64 * t22129;
    let t22131 = -t22098 - t22103 + t22107 + t22111 + t22115 - t22117 - t22119 + t22124 + t22126 - t22128 - t22130;
    (t22124, t22128, t22130, t22131)
}
