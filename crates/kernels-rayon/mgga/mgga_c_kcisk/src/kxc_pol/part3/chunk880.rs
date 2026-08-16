//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 880/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk880(t13206: f64, t5926: f64, t1216: f64, t13176: f64, t13179: f64, t13183: f64, t13187: f64, t13190: f64, t13192: f64, t13194: f64, t13197: f64, t13202: f64, t1421: f64, t3729: f64, t456: f64) -> f64 {
    let t13207 = t5926 * t13206;
    let t13210 = -0.19711289e-2_f64 * t1421 * t13176 + 0.295669335e-2_f64 * t13179 - 12.0_f64 * t1216 * t3729 + 0.39422577999999999999e-2_f64 * t13183 - 0.36958666875e-3_f64 * t456 * t13187 - 0.19711289e-2_f64 * t13190 - 0.59133867e-2_f64 * t13192 + 0.1478346675e-2_f64 * t13194 + 0.65704296666666666667e-3_f64 * t1421 * t13197 - 0.22175200125e-2_f64 * t1421 * t13202 + 0.22175200125e-2_f64 * t1421 * t13207;
    t13210
}
