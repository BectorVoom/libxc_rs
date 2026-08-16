//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1388/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1388(t102985: f64, t102987: f64, t102989: f64, t102991: f64, t102993: f64, t102995: f64, t102997: f64, t102999: f64, t103002: f64, t103004: f64, t103006: f64, t103031: f64, t103033: f64, t103035: f64, t103038: f64, t103040: f64, t103043: f64, t103046: f64, t103049: f64, t103051: f64, t103053: f64, t103056: f64) -> (f64, f64) {
    let t103870 = t102985 / 288.0_f64 - t102987 / 64.0_f64 - t102989 / 9.0_f64 + t102991 / 432.0_f64 + 2.0_f64 / 9.0_f64 * t102993 - t102995 / 36.0_f64 + 19.0_f64 / 72.0_f64 * t102997 - t102999 / 72.0_f64 - t103002 / 32.0_f64 + t103004 / 4.0_f64 + t103006 / 3.0_f64;
    let t103894 = t103031 / 16.0_f64 - t103033 / 8.0_f64 - t103035 / 96.0_f64 - t103038 / 16.0_f64 + t103040 / 3.0_f64 - t103043 / 16.0_f64 + t103046 / 3.0_f64 - 11.0_f64 / 18.0_f64 * t103049 + t103051 / 48.0_f64 + 2.0_f64 / 9.0_f64 * t103053 - 2.0_f64 / 9.0_f64 * t103056;
    (t103870, t103894)
}
