//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 395/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk395(t118: f64, t2471: f64, t2200: f64, t2204: f64, t2382: f64, t2384: f64, t2386: f64, t2388: f64, t2390: f64, t2394: f64, t2464: f64, t2467: f64, t2469: f64) -> f64 {
    let t2472 = t118 * t2471;
    let t2474 = 0.5987120850931904282e-1_f64 * t2382 - 0.8980681276397856423e-1_f64 * t2384 - 0.2993560425465952141e-1_f64 * t2386 - t2200 - 0.20455996240684006298e-1_f64 * t2388 + 0.2727466165424534173e-1_f64 * t2390 + 0.68186654135613354325e-2_f64 * t2394 + t2204 + 0.59871208509319042821e-1_f64 * t2464 - 0.59871208509319042821e-1_f64 * t2467 - 0.39914139006212695214e-1_f64 * t2469 + 0.19957069503106347607e-1_f64 * t2472;
    t2474
}
