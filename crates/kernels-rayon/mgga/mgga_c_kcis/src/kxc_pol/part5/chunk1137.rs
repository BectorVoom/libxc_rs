//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1137/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1137(t19135: f64, t3202: f64, t3200: f64, t167: f64, t1767: f64, t3203: f64, t13172: f64, t6626: f64, t9425: f64, t13122: f64, t14570: f64, t1710: f64, t18511: f64, t18515: f64, t18517: f64, t18521: f64, t18523: f64, t18528: f64, t18532: f64, t19115: f64, t19118: f64, t19121: f64, t19124: f64, t19128: f64, t19130: f64, t19132: f64) -> (f64, f64, f64, f64) {
    let t19136 = t3202 * t19135;
    let t19137 = t3200 * t19136;
    let t19139 = t167 * t1767;
    let t19140 = t3203 * t19139;
    let t19141 = t3202 * t19140;
    let t19142 = t13172 * t19141;
    let t19144 = t9425 * t6626;
    let t19146 = -0.7369753086419753086e-3_f64 * t13122 - 0.33163888888888888888e-2_f64 * t18511 + 0.27636574074074074073e-2_f64 * t18515 + 0.33163888888888888888e-2_f64 * t18517 - 0.13345e0_f64 * t14570 * t1710 - 0.22109259259259259259e-2_f64 * t18521 - 0.66327777777777777776e-2_f64 * t18523 + 0.55273148148148148147e-2_f64 * t18528 - 0.36848765432098765431e-3_f64 * t18532 - 0.24872916666666666666e-2_f64 * t19115 - 0.33163888888888888888e-2_f64 * t19118 - 0.11054629629629629629e-2_f64 * t19121 - 0.88437037037037037035e-2_f64 * t19124 + 0.88437037037037037035e-2_f64 * t19128 - 0.22109259259259259259e-2_f64 * t19130 - 0.33163888888888888888e-2_f64 * t19132 - 0.33163888888888888888e-2_f64 * t19137 - 0.66327777777777777776e-2_f64 * t19142 + 0.22109259259259259258e-2_f64 * t19144;
    (t19137, t19142, t19144, t19146)
}
