//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1022/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1022(t2168: f64, t9194: f64, t9140: f64, t9142: f64, t9143: f64, t9145: f64, t9174: f64, t9175: f64, t9177: f64, t9181: f64, t9183: f64, t9187: f64, t9190: f64, t9192: f64) -> (f64, f64) {
    let t9196 = t2168 * t9194 / 16.0_f64;
    let t9197 = t9140 - t9142 - t9143 - t9145 + t9174 + t9175 - t9177 + t9181 + t9183 + t9187 - t9190 - t9192 - t9196;
    (t9196, t9197)
}
