//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 974/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk974<F: Float>(t1374: F, t7298: F, t8281: F, t1349: F, t23930: F, t2252: F, t342: F, t7302: F, t32679: F, t630: F, t24094: F, t7309: F) -> (F, F, F, F, F) {
    let t138611 = F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t7298 * t8281 * t1374;
    let t138625 = t1349 * t23930;
    let t138629 = t342 * t2252 * t7302 / F::cast_from(18.0_f64);
    let t138635 = t342 * t630 * t32679;
    let t138652 = t7309 * t24094;
    (t138611, t138625, t138629, t138635, t138652)
}
