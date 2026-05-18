//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 311/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk311<F: Float>(t1570: F, t82: F, t1559: F, t356: F, t89: F, t13: F, t360: F) -> (F, F, F, F) {
    let t1571 = t82 * t1570;
    let t1572 = t1571 * t1559;
    let t1574 = t89 * t356 * t1572;
    let t1576 = t360 * t13;
    let t1577 = F::new(1.0) / t1576;
    (t1572, t1574, t1576, t1577)
}
