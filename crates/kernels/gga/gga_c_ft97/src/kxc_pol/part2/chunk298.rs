//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 298/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk298<F: Float>(t133: F, t1354: F, t6: F, t695: F, t224: F, t817: F, t285: F, t342: F, t344: F, t630: F, t11: F, t341: F) -> (F, F, F, F, F, F) {
    let t1355 = t133 * t1354;
    let t1416 = t695 * t6;
    let t1417 = t224 * t1416;
    let t1471 = t817 * t6;
    let t1472 = t285 * t1471;
    let t1524 = t342 * t630 * t344 / F::cast_from(12.0_f64);
    let t1525 = t341 * t11;
    (t1355, t1417, t1471, t1472, t1524, t1525)
}
