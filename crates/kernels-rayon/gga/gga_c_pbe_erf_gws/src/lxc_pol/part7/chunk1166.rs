//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1166/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1166(t6362: f64, t9630: f64, t6289: f64, t6284: f64, t6627: f64, t2157: f64, t2182: f64, t3138: f64, t6177: f64, t6523: f64, t20797: f64, t20799: f64, t20801: f64, t20806: f64, t20808: f64, t20813: f64, t2190: f64, t2306: f64, t2343: f64, t3235: f64, t6282: f64, t902: f64, t905: f64) -> (f64, f64, f64) {
    let t20815 = t9630 * t6362;
    let t20821 = t9630 * t6289;
    let t20823 = t6627 * t6284;
    let t20825 = t2157 * t2182;
    let t20829 = 3.0_f64 / 4.0_f64 * t3138 * t6523 * t6177 * t20825;
    let t20830 = t20797 + t20799 + t20801 - t20806 + t902 * t905 * t20808 * t2306 / 256.0_f64 - 7.0_f64 / 576.0_f64 * t20813 - 7.0_f64 / 64.0_f64 * t20815 - t2343 * t3235 * t6282 * t2190 / 256.0_f64 + 7.0_f64 / 16.0_f64 * t20821 - 7.0_f64 / 48.0_f64 * t20823 + t20829;
    (t20825, t20829, t20830)
}
