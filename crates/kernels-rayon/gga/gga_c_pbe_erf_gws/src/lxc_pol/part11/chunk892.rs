//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 892/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk892(t226: f64, t5903: f64, t1640: f64, t1791: f64, t187: f64, t190: f64, t367: f64, t16704: f64, t1764: f64, t177: f64, t191: f64, t5463: f64, t649: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t17552 = 16.0_f64 / 3.0_f64 * t226 * t5903;
    let t17646 = t1640 * t1791;
    let t17678 = 0.10864197530864197531e0_f64 * t190 * t367 * t187;
    let t17728 = 0.37324691358024691357e0_f64 * t16704;
    let t17758 = t191 / t177 / t1764;
    let t17791 = t5463 * t649;
    (t17552, t17646, t17678, t17728, t17758, t17791)
}
