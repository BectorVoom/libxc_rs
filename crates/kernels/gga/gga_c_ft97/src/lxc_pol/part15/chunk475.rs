//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 475/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk475<F: Float>(t2281: F, t4872: F, t637: F, t2289: F, t3042: F, t4456: F, t4460: F, t4464: F, t4680: F, t4683: F) -> (F, F) {
    let t4874 = t637 * t2281 * t4872;
    let t4883 = -F::cast_from(0.117377e0_f64) * t4680 + F::cast_from(0.234754e0_f64) * t4683 + t2289 + F::cast_from(0.9628722222222222222e-1_f64) * t3042 - F::cast_from(0.9628722222222222222e-1_f64) * t4456 + F::cast_from(0.28886166666666666666e0_f64) * t4460 - F::cast_from(0.14443083333333333333e0_f64) * t4464;
    (t4874, t4883)
}
