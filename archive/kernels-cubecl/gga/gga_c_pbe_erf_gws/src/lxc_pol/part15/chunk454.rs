//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 454/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk454<F: Float>(t633: F, t636: F, t218: F, t648: F) -> (F, F, F) {
    let t1788 = t633 * t636;
    let t1789 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t1788;
    let t1791 = F::cast_from(1.0_f64) / t648 / t218;
    (t1788, t1789, t1791)
}
