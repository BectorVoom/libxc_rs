//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 971/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk971(t1724: f64, t1866: f64, t1620: f64, t1627: f64, t17883: f64, t17911: f64, t1792: f64, t17931: f64, t17964: f64, t17989: f64, t1803: f64, t1809: f64, t1817: f64, t185: f64, t186: f64, t211: f64, t422: f64, t4903: f64, t5048: f64, t5146: f64, t5162: f64, t5352: f64, t5467: f64, t5470: f64, t5524: f64, t617: f64, t626: f64, t650: f64, t663: f64, t7011: f64) -> f64 {
    let t17996 = t1724 * t1724;
    let t18001 = t1866 * t1866;
    let t18008 = 32.0_f64 / 45.0_f64 * t1620 * t1809 * t5048 * t617 - 128.0_f64 / 45.0_f64 * t17883 - 32.0_f64 / 15.0_f64 * t5467 * t1817 - 16.0_f64 / 15.0_f64 * t5470 * t1817 - 16.0_f64 / 15.0_f64 * t1627 * t5146 - 16.0_f64 / 9.0_f64 * t1627 * t5524 + 32.0_f64 / 45.0_f64 * t1620 * t1809 * t5162 * t626 * t422 + 32.0_f64 / 15.0_f64 * t7011 * t4903 - 2.0_f64 / 15.0_f64 * t211 * t186 * t650 * (t17911 + t17931 + t17964 + t17989) + 4.0_f64 / 5.0_f64 * t211 * t186 * t1792 * t17996 + 4.0_f64 / 5.0_f64 * t185 * t186 * t1803 * t18001 - 16.0_f64 / 5.0_f64 * t5352 * t663;
    t18008
}
