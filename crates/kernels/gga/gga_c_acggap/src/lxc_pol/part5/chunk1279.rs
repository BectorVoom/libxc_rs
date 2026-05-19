//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1279/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1279<F: Float>(t3409: F, t5981: F, t1165: F, t1180: F, t18303: F, t18305: F, t18309: F, t18321: F, t18323: F, t18329: F, t18336: F, t18338: F, t18340: F, t1884: F, t3403: F, t407: F, t6100: F, t930: F) -> F {
    let t23666 = t3409 * t5981;
    let t23671 = F::cast_from(0.34299214494455789578e-2_f64) * t18303 + F::cast_from(0.17149607247227894789e-2_f64) * t18305 + F::cast_from(0.10289764348336736873e-1_f64) * t18309 + F::cast_from(0.42874018118069736972e-3_f64) * t18321 - F::cast_from(0.42874018118069736972e-2_f64) * t3403 * t1165 * t1884 * t930 - F::cast_from(0.60023625365297631762e-2_f64) * t18323 + F::cast_from(0.85748036236139473944e-3_f64) * t1180 * t1165 * t6100 * t407 - F::cast_from(0.80031500487063509015e-2_f64) * t18329 - F::cast_from(0.80031500487063509014e-2_f64) * t23666 - F::new(7.0) / F::new(72.0) * t18336 + F::new(7.0) / F::new(36.0) * t18338 + F::new(455.0) / F::new(648.0) * t18340;
    t23671
}
