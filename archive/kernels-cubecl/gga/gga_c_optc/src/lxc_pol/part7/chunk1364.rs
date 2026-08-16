//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1364/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1364<F: Float>(t3147: F, t7878: F, t1133: F, t2586: F, t8517: F, t1121: F, t3128: F, t3137: F, t1111: F, t27096: F, t27101: F, t27105: F, t27110: F, t27113: F, t27119: F, t27124: F, t27127: F, t27131: F, t322: F) -> (F, F, F) {
    let t27134 = t7878 * t3147;
    let t27135 = t1133 * t27134;
    let t27137 = t2586 * t8517;
    let t27138 = t1133 * t27137;
    let t27141 = t1121 * t3137 * t3128;
    let t27143 = -t1111 * t322 * t27096 / F::cast_from(48.0_f64) + t1111 * t322 * t27101 / F::cast_from(6.0_f64) + t1111 * t322 * t27105 / F::cast_from(72.0_f64) + t27110 / F::cast_from(36.0_f64) - t1111 * t322 * t27113 / F::cast_from(12.0_f64) + F::cast_from(0.73258227843678641352e2_f64) * t27119 + F::cast_from(0.18933502127510156893e0_f64) * t27124 + F::cast_from(0.48295341609937543636e-2_f64) * t27127 - F::cast_from(0.96590683219875087274e-1_f64) * t1133 * t27131 - F::cast_from(0.40246118008281286364e-2_f64) * t27135 - F::cast_from(0.48295341609937543636e-1_f64) * t27138 - F::cast_from(0.47333755318775392234e-1_f64) * t27141;
    (t27134, t27137, t27143)
}
