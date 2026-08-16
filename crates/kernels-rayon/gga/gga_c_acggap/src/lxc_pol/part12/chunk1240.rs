//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1240/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1240(t1222: f64, t19834: f64, t20138: f64, t2222: f64, t33163: f64, t33240: f64, t33250: f64, t33735: f64, t38086: f64, t38343: f64, t38345: f64, t38348: f64, t38361: f64, t38370: f64, t4109: f64, t557: f64, t7931: f64, t7934: f64, t8092: f64, t8306: f64, t8400: f64, t9003: f64, t9391: f64, t9427: f64) -> f64 {
    let t38371 = 0.34694512752820797848e1_f64 * t33240 + 0.8673628188205199462e0_f64 * t8400 * t8306 * t19834 - t38343 - t38345 - t38348 + 0.4336814094102599731e0_f64 * t9003 * t8092 - 0.65854491829355115987e0_f64 * t33163 * t557 - 0.17347256376410398924e1_f64 * t8400 * t9427 * t20138 - 0.8673628188205199462e0_f64 * t8400 * t9427 * t33735 + 0.13170898365871023197e1_f64 * t9391 * t1222 + 0.65854491829355115987e0_f64 * t38361 - 0.17347256376410398924e1_f64 * t7931 * t38086 * t7934 - 0.26341796731742046394e1_f64 * t33250 - 0.39512695097613069591e1_f64 * t2222 * t4109 + t38370;
    t38371
}
