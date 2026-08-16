//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1364/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1364(t3147: f64, t7878: f64, t1133: f64, t2586: f64, t8517: f64, t1121: f64, t3128: f64, t3137: f64, t1111: f64, t27096: f64, t27101: f64, t27105: f64, t27110: f64, t27113: f64, t27119: f64, t27124: f64, t27127: f64, t27131: f64, t322: f64) -> (f64, f64, f64) {
    let t27134 = t7878 * t3147;
    let t27135 = t1133 * t27134;
    let t27137 = t2586 * t8517;
    let t27138 = t1133 * t27137;
    let t27141 = t1121 * t3137 * t3128;
    let t27143 = -t1111 * t322 * t27096 / 48.0_f64 + t1111 * t322 * t27101 / 6.0_f64 + t1111 * t322 * t27105 / 72.0_f64 + t27110 / 36.0_f64 - t1111 * t322 * t27113 / 12.0_f64 + 0.73258227843678641352e2_f64 * t27119 + 0.18933502127510156893e0_f64 * t27124 + 0.48295341609937543636e-2_f64 * t27127 - 0.96590683219875087274e-1_f64 * t1133 * t27131 - 0.40246118008281286364e-2_f64 * t27135 - 0.48295341609937543636e-1_f64 * t27138 - 0.47333755318775392234e-1_f64 * t27141;
    (t27134, t27137, t27143)
}
