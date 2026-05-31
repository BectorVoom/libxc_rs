//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 349/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk349<F: Float>(t1243: F, t1251: F, t1365: F, t153: F, t532: F) -> F {
    let t1368 = F::cast_from(0.23e-2_f64) * t1243 + F::cast_from(0.22758333333333333333e-1_f64) * t1251 - F::cast_from(0.60972258698505103132e-2_f64) * t532 + F::cast_from(0.10844166666666666667e-2_f64) * t153 * t1365;
    t1368
}
