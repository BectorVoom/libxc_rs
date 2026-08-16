//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1052/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1052(t169: f64, t5697: f64, t700: f64, t18022: f64, t242: f64, t5701: f64, t1339: f64, t1383: f64, t1452: f64, t39: f64, t4867: f64, t532: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19035 = 0.20752137690161369243e1_f64 * t169 * t5697 * t700;
    let t19037 = t169 * t18022 * t242;
    let t19040 = t169 * t5701 * t700;
    let t19044 = 0.84895108732478328721e0_f64 * t169 * t1339 * t1383;
    let t19045 = t39 * t1452;
    let t19047 = t532 * t4867;
    (t19035, t19037, t19040, t19044, t19045, t19047)
}
