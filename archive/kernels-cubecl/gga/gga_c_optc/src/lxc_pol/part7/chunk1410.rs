//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1410/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1410<F: Float>(t11596: F, t11597: F, t26459: F, t26463: F, t26467: F, t26470: F, t26472: F, t26476: F, t26479: F, t26482: F, t26484: F, t27346: F, t28061: F, t2908: F, t3268: F, t3980: F, t4281: F, t9254: F) -> F {
    let t28063 = F::cast_from(0.31013857721884116596e-1_f64) * t3980 * t2908 * t9254 * t3268 + t26459 + F::cast_from(28.0_f64) / F::cast_from(9.0_f64) * t4281 * t11596 * t11597 * t27346 + t26463 + t26467 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t28061 - t26470 - t26472 + t26476 - t26479 - t26482 + t26484;
    t28063
}
