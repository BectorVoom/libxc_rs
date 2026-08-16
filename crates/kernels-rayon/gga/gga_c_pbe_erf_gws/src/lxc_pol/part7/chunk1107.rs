//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1107/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1107(t2118: f64, t4422: f64, t2362: f64, t822: f64, t2367: f64, t4419: f64, t4386: f64, t4388: f64, t892: f64, t2402: f64, t353: f64, t814: f64, t8599: f64) -> (f64, f64, f64, f64) {
    let t19817 = t2118 * t4422;
    let t19819 = t822 * t19817 * t2362;
    let t19821 = t2367 * t4419;
    let t19824 = t4386 * t892 * t4388;
    let t19829 = t8599 * t353 * t2402 * t814;
    (t19819, t19821, t19824, t19829)
}
