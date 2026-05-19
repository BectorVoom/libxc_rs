//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 594/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk594<F: Float>(t153: F, t2704: F, t2718: F, t39: F, t4573: F) -> F {
    let t4576 = -F::cast_from(0.53666666666666666667e-2_f64) * t2704 - F::cast_from(0.60688888888888888888e-1_f64) * t2718 + F::cast_from(0.1829167760955153094e-1_f64) * t39 - F::cast_from(0.36147222222222222223e-2_f64) * t153 * t4573;
    t4576
}
