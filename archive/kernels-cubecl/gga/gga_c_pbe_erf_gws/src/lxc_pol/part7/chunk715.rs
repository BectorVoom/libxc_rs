//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 715/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk715<F: Float>(t138: F, t1572: F, t1577: F, t1578: F, t1590: F, t514: F, t520: F, t5842: F, t5844: F, t5847: F, t5854: F, t5855: F, t5858: F, t5878: F) -> F {
    let t5880 = t138 * t5842 - F::cast_from(3.0_f64) * t1572 * t1590 + F::cast_from(6.0_f64) * t1577 * t5858 + F::cast_from(6.0_f64) * t1578 * t5847 - t514 * t5878 - F::cast_from(3.0_f64) * t520 * t5844 - F::cast_from(6.0_f64) * t5854 * t5855;
    t5880
}
