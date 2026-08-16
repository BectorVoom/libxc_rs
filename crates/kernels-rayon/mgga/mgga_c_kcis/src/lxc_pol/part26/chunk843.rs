//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 843/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk843(t16937: f64, t5441: f64, t1368: f64, t12140: f64, t498: f64, t5427: f64, t1930: f64, t3967: f64, t1377: f64, t3977: f64, t3754: f64, t1369: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t16938 = t16937 * t5441;
    let t16940 = t1368 * t16938 / 216.0_f64;
    let t16941 = t12140 * t498;
    let t16942 = t16941 * t5427;
    let t16944 = t1368 * t16942 / 324.0_f64;
    let t16954 = t1930 * t3967;
    let t16962 = t3977 * t1377;
    let t16963 = t16962 * t3754;
    let t16968 = t1369 * t1377;
    (t16940, t16941, t16944, t16954, t16962, t16963, t16968)
}
