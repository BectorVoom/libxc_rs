//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 315/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk315<F: Float>(t1342: F, t799: F, t779: F, t1327: F, t803: F) -> (F, F, F) {
    let t1343 = t1342 * t799;
    let t1345 = F::new(1.0) * t779 * t1343;
    let t1347 = -t803 - F::new(0.17123333333333333333e-1) * t1327;
    (t1343, t1345, t1347)
}
