//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 685/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk685(t1733: f64, t649: f64, t661: f64, t1621: f64, t1620: f64, t1622: f64, t1724: f64, t1664: f64, t4352: f64, t590: f64, t587: f64, t1673: f64, t579: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5500 = t649 * t1733;
    let t5501 = t5500 * t661;
    let t5502 = t1621 * t5501;
    let t5504 = 4.0_f64 / 5.0_f64 * t1620 * t5502;
    let t5505 = t1622 * t1724;
    let t5506 = t1621 * t5505;
    let t5508 = 4.0_f64 / 5.0_f64 * t1620 * t5506;
    let t5509 = t1664 * t4352;
    let t5510 = t590 * t5509;
    let t5512 = 8.0_f64 / 15.0_f64 * t587 * t5510;
    let t5513 = t579 * t1673;
    (t5500, t5501, t5502, t5504, t5505, t5506, t5508, t5509, t5510, t5512, t5513)
}
