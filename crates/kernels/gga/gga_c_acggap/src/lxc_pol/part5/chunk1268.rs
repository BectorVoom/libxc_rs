//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1268/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1268<F: Float>(t1165: F, t1180: F, t1181: F, t1532: F, t157: F, t1743: F, t17984: F, t18000: F, t18017: F, t18019: F, t19510: F, t23396: F, t23398: F, t23405: F, t23411: F, t335: F, t336: F, t337: F, t3403: F, t397: F, t398: F, t4875: F, t4919: F, t5852: F, t930: F) -> F {
    let t23420 = -F::cast_from(0.42874018118069736972e-2_f64) * t3403 * t1165 * t5852 * t4919 - F::cast_from(0.42874018118069736972e-3_f64) * t397 * t398 * t1743 * t930 - F::cast_from(0.42874018118069736972e-3_f64) * t23396 + F::cast_from(0.80031500487063509014e-2_f64) * t23398 - F::cast_from(0.85748036236139473944e-3_f64) * t1180 * t1181 * t1532 * t157 * t4875 + F::cast_from(0.85748036236139473944e-3_f64) * t23405 - F::cast_from(0.68598428988911579156e-2_f64) * t17984 - F::cast_from(0.42874018118069736972e-3_f64) * t23411 - F::cast_from(0.25724410870841842183e-2_f64) * t18000 - t335 * t336 * t337 * t19510 / F::cast_from(48.0_f64) - F::cast_from(0.68598428988911579156e-2_f64) * t18017 + F::cast_from(0.34299214494455789578e-2_f64) * t18019;
    t23420
}
