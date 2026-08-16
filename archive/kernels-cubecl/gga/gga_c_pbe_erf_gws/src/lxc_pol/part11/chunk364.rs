//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 364/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk364<F: Float>(t128: F, t1552: F, t1251: F, t1243: F, t502: F, t505: F, t95: F) -> (F, F, F, F) {
    let t1553 = t1552 * t128;
    let t1555 = F::cast_from(0.16322666666666666667e0_f64) * t1553 * t1251;
    let t1561 = F::cast_from(0.32645333333333333333e0_f64) * t502 * t1243;
    let t1563 = F::cast_from(1.0_f64) / t505 / t95;
    (t1553, t1555, t1561, t1563)
}
