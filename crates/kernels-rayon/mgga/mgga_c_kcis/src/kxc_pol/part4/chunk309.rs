//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 309/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk309(t1131: f64, t1133: f64, t1021: f64, t1092: f64, t1010: f64, t1016: f64, t1025: f64, t1089: f64, t1124: f64, t1128: f64, t300: f64, t975: f64, t979: f64) -> (f64, f64, f64, f64) {
    let t1134 = t1131 * t1133;
    let t1135 = t1021 * t1134;
    let t1136 = t1092 * t1135;
    let t1138 = t975 * t300 - 0.66725e-1_f64 * t979 * t1010 + t1016 + 0.16581944444444444444e-2_f64 * t1025 + 0.24872916666666666666e-2_f64 * t1089 - 0.24872916666666666666e-2_f64 * t1124 - 0.66327777777777777776e-2_f64 * t1128 + 0.16581944444444444444e-2_f64 * t1136;
    (t1134, t1135, t1136, t1138)
}
