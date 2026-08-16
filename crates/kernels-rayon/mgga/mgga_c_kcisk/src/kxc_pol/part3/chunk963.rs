//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 963/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk963(t1411: f64, t14236: f64, t14201: f64, t14206: f64, t14211: f64, t14216: f64, t14218: f64, t14220: f64, t14224: f64, t14226: f64, t14228: f64, t14230: f64, t14232: f64) -> (f64, f64) {
    let t14237 = t1411 * t14236;
    let t14239 = 0.2653111111111111111e-1_f64 * t14201 - 0.49745833333333333332e-2_f64 * t14206 + 0.73697530864197530862e-3_f64 * t14211 + 0.44218518518518518518e-2_f64 * t14216 + 0.33163888888888888887e-2_f64 * t14218 - 0.99491666666666666664e-2_f64 * t14220 + 0.16581944444444444444e-2_f64 * t14224 - 0.11054629629629629629e-2_f64 * t14226 - 0.17687407407407407407e-1_f64 * t14228 - 0.66327777777777777776e-2_f64 * t14230 - 0.17687407407407407407e-1_f64 * t14232 + 0.99491666666666666664e-2_f64 * t14237;
    (t14237, t14239)
}
