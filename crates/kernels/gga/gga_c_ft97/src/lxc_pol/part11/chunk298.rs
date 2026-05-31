//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 298/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk298<F: Float>(t1570: F, t82: F, t1559: F, t356: F, t89: F, t13: F, t360: F, t18: F, t361: F) -> (F, F, F, F, F, F) {
    let t1571 = t82 * t1570;
    let t1572 = t1571 * t1559;
    let t1574 = t89 * t356 * t1572;
    let t1576 = t360 * t13;
    let t1577 = F::cast_from(1.0_f64) / t1576;
    let t1578 = t18 * t1577;
    let t1580 = -F::cast_from(2.0_f64) * t361 + F::cast_from(2.0_f64) * t1578;
    (t1571, t1572, t1574, t1576, t1577, t1580)
}
