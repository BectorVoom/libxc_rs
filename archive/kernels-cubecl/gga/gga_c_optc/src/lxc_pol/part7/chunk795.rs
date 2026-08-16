//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 795/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk795<F: Float>(t7524: F, t7525: F, t7527: F, t7529: F, t7531: F, t7535: F, t7538: F, t7541: F, t7544: F, t7547: F, t7550: F, t787: F) -> (F, F) {
    let t7552 = -t7524 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t7525 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t7527 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t7529 + t7531 / F::cast_from(3.0_f64) - F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t7535 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t7538 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t7541 - F::cast_from(2.0_f64) * t7544 + F::cast_from(2.0_f64) * t7547 - t7550 / F::cast_from(3.0_f64);
    let t7553 = t787 * t7552;
    (t7552, t7553)
}
