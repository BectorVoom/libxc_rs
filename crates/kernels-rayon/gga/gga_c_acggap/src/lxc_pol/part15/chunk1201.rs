//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1201/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1201(t2131: f64, t2147: f64, t309: f64, t9985: f64, t10022: f64, t1659: f64, t32124: f64, t33175: f64, t33185: f64, t33201: f64, t38052: f64, t38228: f64, t38232: f64, t38241: f64, t38251: f64, t38256: f64, t38259: f64, t7912: f64, t7931: f64, t8402: f64, t8440: f64, t9391: f64, t9508: f64) -> f64 {
    let t41231 = t2131 * t2147 * t9985 * t309;
    let t41246 = -t38228 + t38232 + 0.34694512752820797848e1_f64 * t41231 + t33185 - 0.13170898365871023197e1_f64 * t9391 * t1659 + 0.52041769129231196772e1_f64 * t32124 * t38052 * t8440 - 0.17347256376410398924e1_f64 * t7931 * t38052 * t8402 + t38241 - t38251 + t38256 - t38259 + 0.4336814094102599731e0_f64 * t7912 * t10022 - 0.17347256376410398924e1_f64 * t7931 * t33175 * t9508 + t33201;
    t41246
}
