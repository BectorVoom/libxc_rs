//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1174/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1174<F: Float>(t24223: F, t24225: F, t24228: F, t24230: F, t24233: F, t24299: F, t24308: F, t24337: F, t24339: F, t24344: F, t24345: F, t2566: F) -> (F, F) {
    let t24347 = t24223 + t24225 + t24228 + t24230 + t24233 + t24299 + t24308 + t24337 + t24339 - t24344 + F::cast_from(200.0_f64) / F::cast_from(9.0_f64) * t24345;
    let t24354 = t2566 * t2566;
    (t24347, t24354)
}
