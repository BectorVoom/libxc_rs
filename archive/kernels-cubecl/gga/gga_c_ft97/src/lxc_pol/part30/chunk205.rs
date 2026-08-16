//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 205/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk205<F: Float>(t14: F, t360: F, t12: F, t13: F) -> (F, F, F, F, F) {
    let t1541 = F::cast_from(1.0_f64) / t14 / t360;
    let t1542 = t12 * t1541;
    let t1576 = t360 * t13;
    let t1577 = F::cast_from(1.0_f64) / t1576;
    let t1609 = F::powf(F::cast_from(4.0_f64), F::cast_from(1.0_f64) / F::cast_from(15.0_f64));
    (t1541, t1542, t1576, t1577, t1609)
}
