//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 654/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk654<F: Float>(t1509: F, t5021: F, t193: F, t670: F, t22: F, t137: F, t647: F) -> (F, F, F, F) {
    let t5022 = t5021 * t1509;
    let t5054 = t670 * t193;
    let t5056 = F::cast_from(1.0_f64) / t22 / t5054;
    let t5059 = t647 * t137;
    (t5022, t5054, t5056, t5059)
}
