//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1071/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1071<F: Float>(t151: F, t2124: F, t2126: F, t2168: F, t22161: F, t22168: F, t22172: F, t22178: F, t22187: F, t22192: F, t22224: F, t22883: F, t23234: F, t23247: F, t23254: F, t23259: F, t23267: F, t23270: F, t3467: F, t6931: F, t7129: F, t9961: F) -> F {
    let t23274 = -F::cast_from(0.48681704342817043984e1_f64) * t23234 + F::cast_from(0.24182738140014814697e0_f64) * t2168 * t22161 + F::cast_from(0.417271751509860377e1_f64) * t9961 * t2126 * t22187 - F::cast_from(0.31295381363239528276e1_f64) * t2124 * t7129 * t22172 + F::cast_from(0.10431793787746509425e1_f64) * t2124 * t2126 * t22224 - F::cast_from(0.36274107210022222046e1_f64) * t2168 * t6931 * t23247 - F::cast_from(0.62590762726479056552e1_f64) * t2124 * t7129 * t23247 + F::cast_from(0.8463958349005185144e0_f64) * t23254 - F::cast_from(0.31295381363239528276e1_f64) * t2124 * t7129 * t22178 - F::cast_from(0.48681704342817043985e1_f64) * t23259 + F::cast_from(0.31295381363239528276e1_f64) * t3467 * t151 * t22883 - F::cast_from(0.31295381363239528276e1_f64) * t9961 * t151 * t22192 - F::cast_from(0.33855833396020740576e1_f64) * t23267 + F::cast_from(0.83454350301972075403e1_f64) * t2124 * t23270 * t22168;
    t23274
}
