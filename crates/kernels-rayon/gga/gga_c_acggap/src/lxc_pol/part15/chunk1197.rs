//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1197/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1197(t119: f64, t9971: f64, t10011: f64, t1914: f64, t2146: f64, t2217: f64, t2404: f64, t33063: f64, t33065: f64, t38092: f64, t38138: f64, t38140: f64, t38647: f64, t38784: f64, t38827: f64, t41089: f64, t464: f64, t7890: f64, t7912: f64, t7931: f64, t8004: f64, t8306: f64, t8400: f64, t9003: f64, t9025: f64, t9058: f64, t9150: f64, t944: f64) -> f64 {
    let t41142 = t119 * t9971;
    let t41145 = -0.52041769129231196772e1_f64 * t9003 * t9150 + 0.65854491829355115987e0_f64 * t33063 + 0.13170898365871023197e1_f64 * t33065 - 0.8673628188205199462e0_f64 * t9058 * t2404 + 0.4336814094102599731e0_f64 * t7912 * t10011 + 0.8673628188205199462e0_f64 * t8400 * t8306 * t38827 + 0.4336814094102599731e0_f64 * t8400 * t8306 * t38647 - 0.17347256376410398924e1_f64 * t7931 * t38092 * t9025 + 0.4336814094102599731e0_f64 * t8400 * t8306 * t38784 - 0.8673628188205199462e0_f64 * t2146 * t7890 * t41089 * t944 - t38138 - 0.26020884564615598386e1_f64 * t2146 * t8004 * t2217 * t1914 - t38140 - 0.65854491829355115987e0_f64 * t41142 * t464;
    t41145
}
