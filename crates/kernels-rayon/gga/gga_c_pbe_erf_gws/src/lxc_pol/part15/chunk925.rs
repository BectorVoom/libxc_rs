//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 925/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk925(t8079: f64, t8082: f64, t8084: f64, t8086: f64, t8088: f64, t8091: f64, t8094: f64, t8096: f64, t8098: f64, t8100: f64, t142: f64, t2873: f64) -> (f64, f64) {
    let t8102 = 4.0_f64 / 27.0_f64 * t8079 - 4.0_f64 / 9.0_f64 * t8082 - t8084 / 9.0_f64 + 2.0_f64 / 3.0_f64 * t8086 - 2.0_f64 * t8088 + 4.0_f64 / 27.0_f64 * t8091 + 4.0_f64 / 9.0_f64 * t8094 - t8096 / 9.0_f64 - 2.0_f64 / 3.0_f64 * t8098 + 2.0_f64 * t8100;
    let t8108 = t142 * t2873;
    (t8102, t8108)
}
