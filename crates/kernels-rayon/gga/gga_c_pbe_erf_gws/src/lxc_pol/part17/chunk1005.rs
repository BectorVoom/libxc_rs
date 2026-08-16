//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1005/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1005(t6481: f64, t9007: f64, t9011: f64, t9015: f64, t9018: f64, t9019: f64, t9021: f64, t9023: f64, t9025: f64, t9030: f64, t9031: f64, t9032: f64) -> (f64, f64) {
    let t9033 = 35.0_f64 / 108.0_f64 * t6481;
    let t9034 = t9007 - t9011 - t9015 + t9018 - t9019 - t9021 - t9023 - t9025 - t9030 + t9031 + t9032 - t9033;
    (t9033, t9034)
}
