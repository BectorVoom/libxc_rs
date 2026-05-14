//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1137/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1137<F: Float>(t1165: F, t3451: F, t4183: F, t5862: F, t3409: F, t5796: F, t12727: F, t1761: F, t1772: F, t368: F, t384: F, t398: F, t879: F, t1180: F, t1181: F, t1532: F, t157: F, t1743: F, t17984: F, t18000: F, t18017: F, t18019: F, t19510: F, t335: F, t336: F, t337: F, t3403: F, t397: F, t4875: F, t4919: F, t5852: F, t930: F) -> (F,) {
    let t23396 = t3451 * t1165 * t5862 * t4183;
    let t23398 = t3409 * t5796;
    let t23405 = t12727 * t1761;
    let t23411 = t384 * t398 * t368 * t1772 * t879;
    let t23420 = -0.42874018118069736972e-2 * t3403 * t1165 * t5852 * t4919 - 0.42874018118069736972e-3 * t397 * t398 * t1743 * t930 - 0.42874018118069736972e-3 * t23396 + 0.80031500487063509014e-2 * t23398 - 0.85748036236139473944e-3 * t1180 * t1181 * t1532 * t157 * t4875 + 0.85748036236139473944e-3 * t23405 - 0.68598428988911579156e-2 * t17984 - 0.42874018118069736972e-3 * t23411 - 0.25724410870841842183e-2 * t18000 - t335 * t336 * t337 * t19510 / 48.0 - 0.68598428988911579156e-2 * t18017 + 0.34299214494455789578e-2 * t18019;
    (t23420,)
}
