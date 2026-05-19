//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 420/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk420<F: Float>(t2281: F, t2282: F, t637: F, t1638: F, t1640: F, t1645: F, t1649: F, t1653: F, t2008: F, t2011: F) -> (F, F) {
    let t2284 = t637 * t2281 * t2282;
    let t2289 = F::cast_from(0.19257444444444444444e0_f64) * t1638;
    let t2294 = -F::new(0.117377e0) * t2008 + F::new(0.234754e0) * t2011 + t2289 + F::cast_from(0.9628722222222222222e-1_f64) * t1640 - F::cast_from(0.9628722222222222222e-1_f64) * t1645 + F::cast_from(0.28886166666666666666e0_f64) * t1649 - F::cast_from(0.14443083333333333333e0_f64) * t1653;
    (t2284, t2294)
}
