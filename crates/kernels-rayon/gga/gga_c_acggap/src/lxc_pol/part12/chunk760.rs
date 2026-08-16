//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 760/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk760(t7890: f64, t8117: f64, t944: f64, t2146: f64, t2236: f64, t2241: f64, t7912: f64, t8062: f64, t8067: f64, t8069: f64, t8074: f64, t8076: f64, t8078: f64, t8082: f64, t8087: f64, t8092: f64, t8096: f64, t8098: f64, t8101: f64, t8106: f64, t8108: f64, t8113: f64, t8115: f64) -> (f64, f64) {
    let t8119 = t7890 * t8117 * t944;
    let t8122 = 0.17347256376410398924e1_f64 * t7912 * t2236 + 0.17347256376410398924e1_f64 * t8062 + t8067 + 0.17347256376410398924e1_f64 * t2146 * t8069 + 0.34694512752820797848e1_f64 * t8074 + 0.17347256376410398924e1_f64 * t8076 - 0.17347256376410398924e1_f64 * t8078 - 0.34694512752820797848e1_f64 * t8082 + t8087 + 0.8673628188205199462e0_f64 * t7912 * t2241 + 0.4336814094102599731e0_f64 * t2146 * t8092 - t8096 - t8098 - 0.17347256376410398924e1_f64 * t8101 - t8106 - 0.26020884564615598386e1_f64 * t2146 * t8108 - t8113 - 0.13170898365871023197e1_f64 * t8115 - 0.8673628188205199462e0_f64 * t2146 * t8119;
    (t8119, t8122)
}
