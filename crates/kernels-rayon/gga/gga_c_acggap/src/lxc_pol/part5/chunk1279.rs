//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1279/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1279(t3409: f64, t5981: f64, t1165: f64, t1180: f64, t18303: f64, t18305: f64, t18309: f64, t18321: f64, t18323: f64, t18329: f64, t18336: f64, t18338: f64, t18340: f64, t1884: f64, t3403: f64, t407: f64, t6100: f64, t930: f64) -> f64 {
    let t23666 = t3409 * t5981;
    let t23671 = 0.34299214494455789578e-2_f64 * t18303 + 0.17149607247227894789e-2_f64 * t18305 + 0.10289764348336736873e-1_f64 * t18309 + 0.42874018118069736972e-3_f64 * t18321 - 0.42874018118069736972e-2_f64 * t3403 * t1165 * t1884 * t930 - 0.60023625365297631762e-2_f64 * t18323 + 0.85748036236139473944e-3_f64 * t1180 * t1165 * t6100 * t407 - 0.80031500487063509015e-2_f64 * t18329 - 0.80031500487063509014e-2_f64 * t23666 - 7.0_f64 / 72.0_f64 * t18336 + 7.0_f64 / 36.0_f64 * t18338 + 455.0_f64 / 648.0_f64 * t18340;
    t23671
}
