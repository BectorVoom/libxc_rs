//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1082/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1082<F: Float>(t352: F, t6759: F, t1337: F, t1347: F, t18806: F, t89: F, t124: F, t1465: F, t1468: F, t625: F) -> (F, F, F, F, F) {
    let t19305 = t352 * t6759;
    let t19309 = 1.0 / t1347 / t1337;
    let t19326 = t1347 * t1347;
    let t19327 = 1.0 / t19326;
    let t19336 = t18806 * t89;
    let t19341 = 0.28493333333333333333e0 * t625 * t124 * t1465 * t1468;
    (t19305, t19309, t19327, t19336, t19341)
}
