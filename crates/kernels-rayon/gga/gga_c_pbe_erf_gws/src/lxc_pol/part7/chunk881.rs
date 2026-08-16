//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 881/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk881(t1413: f64, t1697: f64, t1793: f64, t4927: f64, t639: f64, t4923: f64, t5129: f64, t587: f64, t1765: f64, t1804: f64, t5548: f64, t1672: f64, t185: f64, t1867: f64) -> (f64, f64, f64, f64) {
    let t16811 = 32.0_f64 / 15.0_f64 * t639 * t4927 * t1793 * t1697 * t1413;
    let t16813 = t587 * t5129 * t4923;
    let t16814 = 64.0_f64 / 45.0_f64 * t16813;
    let t16818 = 32.0_f64 / 15.0_f64 * t587 * t5548 * t1765 * t1804;
    let t16820 = t185 * t1672 * t1867;
    (t16811, t16814, t16818, t16820)
}
