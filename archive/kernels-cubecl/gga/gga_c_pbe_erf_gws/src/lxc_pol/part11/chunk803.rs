//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 803/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk803<F: Float>(t10134: F, t12962: F, t12970: F, t12973: F, t12987: F, t138: F, t1577: F, t2902: F, t3675: F, t3683: F, t514: F, t5854: F, t8209: F, t985: F) -> F {
    let t12989 = -F::cast_from(3.0_f64) * t10134 * t985 + t12962 * t138 - F::cast_from(6.0_f64) * t12970 * t5854 + F::cast_from(6.0_f64) * t12973 * t1577 - t12987 * t514 - F::cast_from(3.0_f64) * t2902 * t3683 + F::cast_from(6.0_f64) * t3675 * t8209;
    t12989
}
