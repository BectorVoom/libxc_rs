//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1205/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1205(t1659: f64, t20138: f64, t2127: f64, t2155: f64, t2338: f64, t32196: f64, t32201: f64, t32210: f64, t32219: f64, t33566: f64, t35324: f64, t36433: f64, t36498: f64, t36504: f64, t36511: f64, t36515: f64, t36526: f64, t5340: f64, t7879: f64, t7931: f64, t7932: f64, t7934: f64, t8001: f64, t8400: f64, t9033: f64) -> f64 {
    let t36528 = -0.17347256376410398924e1_f64 * t8400 * t9033 * t20138 - t36498 - 0.17347256376410398924e1_f64 * t7931 * t36433 * t7934 + 0.17347256376410398924e1_f64 * t32196 + 0.8673628188205199462e0_f64 * t32201 - 0.65854491829355115987e0_f64 * t36504 - 0.4336814094102599731e0_f64 * t2338 * t7879 + 0.8673628188205199462e0_f64 * t33566 * t2155 - t32210 - 0.17347256376410398924e1_f64 * t7931 * t7932 * t36511 + 0.26020884564615598386e1_f64 * t8400 * t36515 * t35324 + 0.13170898365871023197e1_f64 * t2127 * t5340 + 0.17347256376410398924e1_f64 * t32219 - 0.13170898365871023197e1_f64 * t8001 * t1659 - 0.26020884564615598386e1_f64 * t36526;
    t36528
}
