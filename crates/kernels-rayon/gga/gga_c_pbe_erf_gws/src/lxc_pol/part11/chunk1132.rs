//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1132/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1132(t47928: f64, t48043: f64, t48044: f64, t48045: f64, t48046: f64, t48049: f64, t48050: f64, t48052: f64, t48056: f64, t48059: f64, t48060: f64, t48062: f64) -> f64 {
    let t48063 = t47928 - t48043 - t48044 + t48045 - t48046 + t48049 - t48050 - t48052 - t48056 + t48059 + t48060 - t48062;
    t48063
}
