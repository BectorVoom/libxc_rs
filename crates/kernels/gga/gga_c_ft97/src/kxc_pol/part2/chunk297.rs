//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 297/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk297<F: Float>(t1268: F, t898: F, t900: F, t1263: F, t631: F, t892: F, t332: F, t113: F, t409: F, t6: F, t64: F, t550: F) -> (F, F, F, F, F, F) {
    let t1270 = t898 * t900 * t1268;
    let t1273 = t892 + t631 * t1263 / F::cast_from(6.0_f64) + t631 * t1270 / F::cast_from(2.0_f64);
    let t1274 = t1273 * t332;
    let t1275 = t1274 * t113;
    let t1299 = t409 * t6;
    let t1300 = t64 * t1299;
    let t1354 = t550 * t6;
    (t1270, t1273, t1274, t1275, t1300, t1354)
}
