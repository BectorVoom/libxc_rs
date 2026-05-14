//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 594/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk594<F: Float>(t1305: F, t172: F, t1265: F, t158: F, t475: F, t599: F, t1328: F, t1323: F, t203: F, t123: F, t594: F, t160: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4144 = t172 * t1305;
    let t4149 = t158 * t1265;
    let t4167 = t599 * t475;
    let t4183 = t599 * t1328;
    let t4245 = t172 * t1323;
    let t4250 = t158 * t1328;
    let t4255 = t203 * t1328;
    let t4260 = t594 * t123;
    let t4261 = t4260 * t160;
    (t4144, t4149, t4167, t4183, t4245, t4250, t4255, t4260, t4261)
}
