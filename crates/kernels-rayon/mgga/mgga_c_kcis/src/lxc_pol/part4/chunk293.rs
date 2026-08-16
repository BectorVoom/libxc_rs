//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 293/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk293(t1079: f64, t922: f64, t1030: f64, t104: f64, t1050: f64, t1055: f64, t1057: f64, t1063: f64, t1065: f64, t1069: f64, t1072: f64, t1078: f64, t111: f64, t120: f64, t829: f64) -> (f64, f64) {
    let t1080 = t1079 * t922;
    let t1083 = t1050 + 0.11955719325063177623e-1_f64 * t1030 * t829 - t1055 - 0.3513e-2_f64 * t104 * t1057 + t1063 + 0.7925e-3_f64 * t111 * t1065 - t1069 - 0.5179538907796306876e-4_f64 * t1072 * t829 + t1078 + 0.50413125e-5_f64 * t120 * t1080;
    (t1080, t1083)
}
