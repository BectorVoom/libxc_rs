//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 670/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk670<F: Float>(t159: F, t7089: F, t148: F, t151: F, t6568: F, t2029: F, t2167: F) -> (F, F, F) {
    let t7091 = 0.13322897401211865505e1 * t159 * t7089;
    let t7094 = 0.29299173910028776472e1 * t148 * t6568 * t151;
    let t7110 = t2167 * t2029;
    (t7091, t7094, t7110)
}
