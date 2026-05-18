//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 1053/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk1053<F: Float>(t43409: F, t43412: F, t43414: F, t43416: F, t43426: F, t43433: F, t43440: F, t47145: F, t47149: F, t47151: F, t47155: F, t47157: F, t47160: F, t47164: F, t47166: F, t47170: F, t47174: F, t47180: F, t47186: F, t47191: F) -> F {
    let t51092 = F::new(0.38342925953920749676e0) * t47145 - F::new(0.85206502119823888169e-1) * t47149 - F::new(0.38342925953920749676e0) * t47151 + t43409 - F::new(0.76685851907841499352e0) * t43412 + t43414 - F::new(0.76685851907841499352e0) * t43416 + t47155 + t47157 + t47160 - t47164 + F::new(0.19171462976960374838e1) * t47166 - F::new(0.11502877786176224903e1) * t47170 + F::new(0.20449560508757733161e1) * t47174 - t43426 - t43433 - F::new(0.89376224879626066674e-1) * t47180 - t47186 + t47191 + t43440;
    t51092
}
