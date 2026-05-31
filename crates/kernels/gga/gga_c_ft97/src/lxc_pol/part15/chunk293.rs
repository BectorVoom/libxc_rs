//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 293/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk293<F: Float>(t1570: F, t82: F, t13: F, t360: F, t18: F, t368: F, t81: F) -> (F, F, F, F, F) {
    let t1571 = t82 * t1570;
    let t1576 = t360 * t13;
    let t1577 = F::cast_from(1.0_f64) / t1576;
    let t1578 = t18 * t1577;
    let t1585 = t368 * t81;
    let t1586 = F::cast_from(1.0_f64) / t1585;
    (t1571, t1576, t1577, t1578, t1586)
}
