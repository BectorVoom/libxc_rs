//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 333/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk333<F: Float>(t2247: F, t70: F, t170: F, t180: F, t11: F, t625: F) -> (F, F, F) {
    let t2248 = t2247 * t70;
    let t2251 = F::cast_from(5.0_f64) / F::cast_from(18.0_f64) * t170 * t2248 * t180;
    let t2252 = t11 * t625;
    (t2248, t2251, t2252)
}
