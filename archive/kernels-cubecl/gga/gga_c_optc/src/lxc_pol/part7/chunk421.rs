//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 421/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk421<F: Float>(t2048: F, t88: F, t1834: F, t1838: F, t1981: F, t1985: F, t1988: F, t1992: F, t1996: F, t2000: F, t2044: F, t2047: F) -> F {
    let t2050 = F::cast_from(32.0_f64) * t2048 * t88;
    let t2051 = -t1996 - t2000 - t1988 + t2044 + t2047 - t2050 - t1834 + t1992 - t1981 - t1838 + t1985;
    t2051
}
