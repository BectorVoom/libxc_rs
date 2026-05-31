//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 676/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk676<F: Float>(t1923: F, t247: F, t24: F, t712: F, t2704: F, t2718: F, t248: F, t256: F, t1910: F, t723: F, t1924: F, t1917: F, t245: F) -> (F, F, F, F, F, F, F, F, F) {
    let t5420 = t247 * t1923;
    let t5421 = t24 * t5420;
    let t5423 = F::cast_from(0.18233333333333333333e0_f64) * t712 * t5421;
    let t5426 = F::cast_from(0.10059259259259259259e0_f64) * t2704 - F::cast_from(0.50074074074074074075e0_f64) * t2718;
    let t5427 = t248 * t5426;
    let t5429 = t5427 * t256 / F::cast_from(3.0_f64);
    let t5430 = t1910 * t723;
    let t5433 = F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t1924 * t723;
    let t5434 = t245 * t1917;
    (t5420, t5421, t5423, t5426, t5427, t5429, t5430, t5433, t5434)
}
