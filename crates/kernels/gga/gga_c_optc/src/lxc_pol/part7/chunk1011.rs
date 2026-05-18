//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1011/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1011<F: Float>(t22120: F, t587: F, t601: F, t6405: F, t2204: F, t2229: F, t1998: F, t6632: F, t1994: F, t22098: F, t22103: F, t22107: F, t22111: F, t22115: F, t22117: F, t22119: F) -> (F, F, F, F) {
    let t22124 = F::new(0.1403573615389248977e2) * t601 * t6405 * t22120 * t587;
    let t22126 = F::new(70.0) / F::new(3.0) * t2229 * t2204;
    let t22127 = t6632 * t1998;
    let t22128 = F::new(0.35089340384731224426e1) * t22127;
    let t22129 = t6632 * t1994;
    let t22130 = F::new(0.1038945353962551798e3) * t22129;
    let t22131 = -t22098 - t22103 + t22107 + t22111 + t22115 - t22117 - t22119 + t22124 + t22126 - t22128 - t22130;
    (t22124, t22128, t22130, t22131)
}
