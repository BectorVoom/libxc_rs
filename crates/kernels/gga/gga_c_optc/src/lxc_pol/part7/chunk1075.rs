//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1075/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1075<F: Float>(t102: F, t108: F, t176: F, t203: F, t23331: F, t1909: F, t188: F, t1916: F, t6756: F, t1953: F, t1972: F, t201: F, t2193: F, t22285: F, t22290: F, t22293: F, t23287: F, t23289: F, t23291: F, t3308: F, t5: F, t6766: F, t743: F) -> F {
    let t23336 = t176 * t23331 * t102 * t108 * t203 / F::new(2.0);
    let t23341 = t1909 * t1909;
    let t23348 = t188 * t1916 * t6756;
    let t23350 = t1953 * t1953;
    let t23356 = t22285 + F::new(2.0) * t23287 + F::new(140.0) / F::new(3.0) * t23289 + F::new(70.0) / F::new(3.0) * t23291 - t22290 + t22293 + t23336 + F::cast_from(0.31013857721884116596e-1_f64) * t3308 * t1972 * t6766 * t2193 + t188 * t743 * t5 * t23341 * t201 / F::new(2.0) - F::new(14.0) / F::new(3.0) * t23348 + F::new(3.0) / F::new(2.0) * t188 * t743 * t5 * t23350 * t201;
    t23356
}
