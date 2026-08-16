//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1121/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1121(t42201: f64, t42204: f64, t42206: f64, t42217: f64, t942: f64, t9639: f64, t1550: f64, t2435: f64, t26291: f64, t26387: f64, t36998: f64, t37000: f64, t38125: f64, t42196: f64, t42199: f64, t42211: f64, t42215: f64, t42222: f64, t44277: f64, t5204: f64, t5211: f64, t530: f64, t699: f64, t903: f64) -> f64 {
    let t44423 = 0.1454648621559751559e0_f64 * t42201;
    let t44424 = 0.35754263910370185096e-3_f64 * t42204;
    let t44425 = 0.23836175940246790064e-3_f64 * t42206;
    let t44428 = 0.11918087970123395032e-3_f64 * t42217;
    let t44431 = 0.4726e1_f64 * t942 * t9639;
    let t44440 = -0.2363e1_f64 * t530 * t38125 - 0.71845450211182851384e0_f64 * t26291 * t44277 - 0.26668558061928778581e0_f64 * t42196 + 0.39914139006212695214e-1_f64 * t26387 * t2435 - 0.13637330827122670865e0_f64 * t42199 - t44423 + t44424 - t44425 + 0.5107751987195740728e-4_f64 * t42211 - 0.5107751987195740728e-4_f64 * t42215 + t44428 - 0.1702583995731913576e-4_f64 * t42222 - t44431 + 0.15965655602485078085e0_f64 * t36998 - 0.23948483403727617128e0_f64 * t1550 * t699 * t5204 + 0.35922725105591425692e0_f64 * t903 * t699 * t5211 - 0.79828278012425390427e-1_f64 * t37000;
    t44440
}
