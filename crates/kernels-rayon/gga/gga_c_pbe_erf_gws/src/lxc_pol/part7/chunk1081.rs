//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1081/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1081(t138: f64, t1572: f64, t1577: f64, t1578: f64, t1590: f64, t19258: f64, t19304: f64, t19347: f64, t19386: f64, t19390: f64, t19393: f64, t19398: f64, t19407: f64, t19408: f64, t19414: f64, t19426: f64, t19451: f64, t514: f64, t520: f64, t5844: f64, t5847: f64, t5854: f64, t5855: f64, t5858: f64, t5878: f64) -> f64 {
    let t19454 = (t19258 + t19304 + t19347 + t19386) * t138 - 4.0_f64 * t19390 * t520 + 12.0_f64 * t19393 * t1578 - 6.0_f64 * t5844 * t1590 - 24.0_f64 * t19398 * t5855 + 24.0_f64 * t5847 * t5858 - 4.0_f64 * t1572 * t5878 + 24.0_f64 * t19407 * t19408 - 36.0_f64 * t5854 * t1578 * t1590 + 6.0_f64 * t1577 * t19414 + 8.0_f64 * t1577 * t520 * t5878 - t514 * (t19426 + t19451);
    t19454
}
