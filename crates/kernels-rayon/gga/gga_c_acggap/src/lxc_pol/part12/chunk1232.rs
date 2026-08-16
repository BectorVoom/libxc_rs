//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1232/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1232(t33110: f64, t33114: f64, t33118: f64, t33120: f64, t33124: f64, t33128: f64, t33132: f64, t33138: f64, t38181: f64, t38185: f64, t38187: f64, t38190: f64, t38194: f64, t464: f64, t8342: f64, t9003: f64) -> f64 {
    let t38204 = -0.69389025505641595696e1_f64 * t38181 + 0.34694512752820797848e1_f64 * t38185 - 0.13170898365871023197e1_f64 * t38187 * t464 + 0.17347256376410398924e1_f64 * t38190 - 0.8673628188205199462e0_f64 * t33110 - t38194 - 0.8673628188205199462e0_f64 * t33114 - 0.52041769129231196772e1_f64 * t33118 - 0.52041769129231196772e1_f64 * t33120 - 0.52041769129231196772e1_f64 * t33124 + 0.8673628188205199462e0_f64 * t9003 * t8342 + 0.52041769129231196772e1_f64 * t33128 - 0.17347256376410398924e1_f64 * t33132 + 0.17347256376410398924e1_f64 * t33138;
    t38204
}
