//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 721/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk721(t1494: f64, t531: f64, t1497: f64, t833: f64, t4170: f64, t4160: f64, t1396: f64, t3801: f64, t1395: f64, t1394: f64, t3951: f64, t4117: f64, t4127: f64, t4132: f64, t4139: f64, t4143: f64, t4146: f64, t4150: f64, t4156: f64, t4167: f64, t507: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4171 = t1494 * t531;
    let t4172 = t833 * t1497;
    let t4173 = t4171 * t4172;
    let t4174 = t4170 * t4173;
    let t4175 = t4160 * t4174;
    let t4177 = t1396 * t3801;
    let t4178 = t1395 * t4177;
    let t4179 = t1394 * t4178;
    let t4181 = 0.33163888888888888888e-2_f64 * t4117 + t3951 * t507 + 0.49745833333333333332e-2_f64 * t4127 - 0.33163888888888888888e-2_f64 * t4132 - 0.55273148148148148147e-3_f64 * t4139 + 0.22109259259259259258e-2_f64 * t4143 + 0.33163888888888888888e-2_f64 * t4146 + 0.16581944444444444444e-2_f64 * t4150 + 0.27636574074074074073e-2_f64 * t4156 - 0.33163888888888888888e-2_f64 * t4167 + 0.22109259259259259258e-2_f64 * t4175 - 0.33163888888888888888e-2_f64 * t4179;
    (t4171, t4172, t4173, t4174, t4175, t4177, t4178, t4179, t4181)
}
