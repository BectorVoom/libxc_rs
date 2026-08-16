//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1268/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1268(t1165: f64, t1180: f64, t1181: f64, t1532: f64, t157: f64, t1743: f64, t17984: f64, t18000: f64, t18017: f64, t18019: f64, t19510: f64, t23396: f64, t23398: f64, t23405: f64, t23411: f64, t335: f64, t336: f64, t337: f64, t3403: f64, t397: f64, t398: f64, t4875: f64, t4919: f64, t5852: f64, t930: f64) -> f64 {
    let t23420 = -0.42874018118069736972e-2_f64 * t3403 * t1165 * t5852 * t4919 - 0.42874018118069736972e-3_f64 * t397 * t398 * t1743 * t930 - 0.42874018118069736972e-3_f64 * t23396 + 0.80031500487063509014e-2_f64 * t23398 - 0.85748036236139473944e-3_f64 * t1180 * t1181 * t1532 * t157 * t4875 + 0.85748036236139473944e-3_f64 * t23405 - 0.68598428988911579156e-2_f64 * t17984 - 0.42874018118069736972e-3_f64 * t23411 - 0.25724410870841842183e-2_f64 * t18000 - t335 * t336 * t337 * t19510 / 48.0_f64 - 0.68598428988911579156e-2_f64 * t18017 + 0.34299214494455789578e-2_f64 * t18019;
    t23420
}
