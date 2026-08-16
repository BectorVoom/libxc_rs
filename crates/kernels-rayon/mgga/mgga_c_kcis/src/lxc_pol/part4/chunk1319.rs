//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1319/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1319(t3970: f64, t498: f64, t5441: f64, t1368: f64, t12140: f64, t5427: f64, t2645: f64, t5721: f64, t3984: f64, t1938: f64, t3754: f64, t2642: f64) -> (f64, f64, f64, f64) {
    let t16937 = t3970 * t498;
    let t16938 = t16937 * t5441;
    let t16940 = t1368 * t16938 / 216.0_f64;
    let t16941 = t12140 * t498;
    let t16942 = t16941 * t5427;
    let t16944 = t1368 * t16942 / 324.0_f64;
    let t16945 = t5721 * t2645;
    let t16946 = t3984 * t16945;
    let t16949 = t1938 * t3754;
    let t16950 = t16949 * t2642;
    (t16940, t16944, t16946, t16950)
}
