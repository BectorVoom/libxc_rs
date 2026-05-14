//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 275/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk275<F: Float>(t27: F, t338: F, t13: F, t355: F, t356: F) -> (F, F) {
    let t1132 = t338 * t27;
    let t1133 = 1.0 / t1132;
    let t1134 = t13 * t1133;
    let t1135 = t355 * t355;
    let t1136 = t1135 * t356;
    let t1138 = 2.0 * t1134 * t1136;
    (t1135, t1138)
}
