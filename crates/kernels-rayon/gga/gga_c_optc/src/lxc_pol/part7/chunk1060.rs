//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1060/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1060(t146: f64, t2002: f64, t2111: f64, t2116: f64, t108: f64, t6990: f64, t110: f64, t2164: f64, t7012: f64, t22154: f64, t56: f64, t148: f64, t151: f64) -> (f64, f64, f64, f64, f64) {
    let t23027 = t146 * t2111 * t2002;
    let t23028 = t23027 * t2116;
    let t23038 = t6990 * t108;
    let t23040 = t146 * t23038 * t110;
    let t23045 = t2164 * t7012;
    let t23047 = t22154 * t56;
    let t23050 = 0.15626226085348680785e2_f64 * t148 * t23047 * t151;
    (t23028, t23040, t23045, t23047, t23050)
}
