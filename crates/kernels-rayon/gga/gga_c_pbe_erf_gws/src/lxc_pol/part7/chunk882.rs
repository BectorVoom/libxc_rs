//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 882/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk882(t16820: f64, t218: f64, t5108: f64, t213: f64, t1793: f64, t186: f64, t211: f64, t16781: f64, t16787: f64, t16792: f64, t16796: f64, t16800: f64, t16806: f64, t16811: f64, t16814: f64, t16818: f64) -> (f64, f64, f64) {
    let t16821 = 8.0_f64 / 45.0_f64 * t16820;
    let t16823 = 1.0_f64 / t5108 / t218;
    let t16824 = t213 * t16823;
    let t16825 = t1793 * t1793;
    let t16829 = 16.0_f64 / 5.0_f64 * t211 * t186 * t16824 * t16825;
    let t16830 = -t16781 - t16787 + t16792 + t16796 + t16800 - t16806 - t16811 + t16814 - t16818 + t16821 + t16829;
    (t16821, t16829, t16830)
}
