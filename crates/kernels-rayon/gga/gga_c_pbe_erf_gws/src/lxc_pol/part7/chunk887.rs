//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 887/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk887(t1413: f64, t1642: f64, t1724: f64, t5522: f64, t639: f64, t1464: f64, t671: f64, t1457: f64, t4892: f64, t5129: f64, t587: f64, t1820: f64, t4919: f64, t5125: f64) -> (f64, f64, f64, f64, f64) {
    let t16874 = 8.0_f64 / 9.0_f64 * t639 * t5522 * t1724 * t1642 * t1413;
    let t16876 = 0.44134814814814814812e-2_f64 * t1464 * t671;
    let t16877 = t1457 * t671;
    let t16880 = t587 * t5129 * t4892;
    let t16881 = 32.0_f64 / 45.0_f64 * t16880;
    let t16883 = t1820 * t5125 * t4919;
    (t16874, t16876, t16877, t16881, t16883)
}
