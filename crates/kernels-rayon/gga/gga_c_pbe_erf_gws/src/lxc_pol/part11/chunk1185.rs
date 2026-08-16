//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1185/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1185(t41339: f64, t48067: f64, t48069: f64, t48071: f64, t48076: f64, t48078: f64, t48080: f64, t48082: f64, t48084: f64, t48086: f64, t48088: f64, t17548: f64, t26328: f64, t48090: f64, t48092: f64, t48095: f64, t48099: f64, t48101: f64, t48102: f64, t48103: f64, t48104: f64, t48105: f64) -> (f64, f64) {
    let t48667 = t48067 + t48069 + t48071 + t48076 + t48078 + t48080 + 16.0_f64 / 3.0_f64 * t41339 - t48082 - t48084 + t48086 + t48088;
    let t48669 = -t48090 + t48092 - t48095 - t48099 - t48101 + 16.0_f64 / 3.0_f64 * t26328 - t48102 + t48103 - t48104 - t48105 + t17548;
    (t48667, t48669)
}
