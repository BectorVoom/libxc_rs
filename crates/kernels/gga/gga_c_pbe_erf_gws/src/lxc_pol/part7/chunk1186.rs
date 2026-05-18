//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1186/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1186<F: Float>(t2142: F, t6488: F, t20802: F, t875: F, t2168: F, t4386: F, t6084: F, t817: F, t2100: F, t2106: F, t6095: F, t814: F) -> (F, F, F, F, F, F) {
    let t21063 = t6488 * t2142;
    let t21064 = F::new(7.0) / F::new(36.0) * t21063;
    let t21065 = t875 * t20802;
    let t21068 = t2168 * t4386 * t21065 / F::new(4.0);
    let t21074 = t6084 * t817;
    let t21077 = t2100 * t2106;
    let t21082 = t814 * t6095;
    (t21064, t21065, t21068, t21074, t21077, t21082)
}
