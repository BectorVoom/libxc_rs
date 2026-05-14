//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 685/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk685<F: Float>(t1494: F, t531: F, t1497: F, t833: F, t4170: F, t4160: F, t1396: F, t3801: F, t1395: F, t1394: F, t3951: F, t4117: F, t4127: F, t4132: F, t4139: F, t4143: F, t4146: F, t4150: F, t4156: F, t4167: F, t507: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4171 = t1494 * t531;
    let t4172 = t833 * t1497;
    let t4173 = t4171 * t4172;
    let t4174 = t4170 * t4173;
    let t4175 = t4160 * t4174;
    let t4177 = t1396 * t3801;
    let t4178 = t1395 * t4177;
    let t4179 = t1394 * t4178;
    let t4181 = 0.33163888888888888888e-2 * t4117 + t3951 * t507 + 0.49745833333333333332e-2 * t4127 - 0.33163888888888888888e-2 * t4132 - 0.55273148148148148147e-3 * t4139 + 0.22109259259259259258e-2 * t4143 + 0.33163888888888888888e-2 * t4146 + 0.16581944444444444444e-2 * t4150 + 0.27636574074074074073e-2 * t4156 - 0.33163888888888888888e-2 * t4167 + 0.22109259259259259258e-2 * t4175 - 0.33163888888888888888e-2 * t4179;
    (t4171, t4172, t4173, t4174, t4175, t4177, t4178, t4179, t4181)
}
