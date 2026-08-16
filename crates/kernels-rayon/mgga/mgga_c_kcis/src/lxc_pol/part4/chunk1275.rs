//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1275/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1275(t11388: f64, t1919: f64, t4374: f64, t16144: f64, t16048: f64, t11409: f64, t11411: f64, t11413: f64, t11415: f64, t11455: f64, t11457: f64, t11460: f64, t16050: f64, t16062: f64, t16088: f64) -> (f64, f64, f64) {
    let t16280 = t11388 * t1919;
    let t16281 = t16280 * t4374;
    let t16292 = 0.22076e0_f64 * t16144;
    let t16301 = 0.13418888888888888889e0_f64 * t16048;
    let t16306 = -0.26837777777777777778e0_f64 * t11409 + 0.67094444444444444447e-1_f64 * t11411 - 0.20128333333333333334e0_f64 * t11413 + 0.10064166666666666667e0_f64 * t11415 + 0.60385e0_f64 * t16088 + 0.12077e1_f64 * t16062 + t16301 - 0.40256666666666666667e0_f64 * t16050 - 0.18396666666666666667e0_f64 * t11455 + 0.5519e-1_f64 * t11457 + 0.18396666666666666667e-1_f64 * t11460;
    (t16281, t16292, t16306)
}
