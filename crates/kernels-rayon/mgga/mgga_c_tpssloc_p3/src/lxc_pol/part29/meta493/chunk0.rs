//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1844/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1844(t3252: f64, t7286: f64, t7285: f64, t3248: f64, t24574: f64, t7288: f64, t225: f64, t7306: f64, t2154: f64, t3599: f64, t11606: f64, t11925: f64, t11928: f64, t1238: f64, t1252: f64, t2155: f64, t24630: f64, t24634: f64, t24639: f64, t24646: f64, t24758: f64, t24868: f64, t24871: f64, t24873: f64, t24877: f64, t24880: f64, t3593: f64, t3631: f64, t498: f64, t7283: f64, t7351: f64, t7392: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t24883 = t7286 * t3252;
    let t24884 = t7285 * t24883;
    let t24887 = t7286 * t3248;
    let t24888 = t7285 * t24887;
    let t24891 = t24574 * t7288;
    let t24893 = t7306 * t225;
    let t24896 = t2154 * t3599;
    let t24897 = t11606 * t24896;
    let t24900 = -0.16449340668482264365e-1_f64 * t7283 * t24630 - 0.54831135561607547884e-2_f64 * t7283 * t24634 + 0.16449340668482264365e-1_f64 * t7283 * t24639 - t11928 * t2155 - 2.0_f64 * t3593 * t7392 + 0.54831135561607547884e-2_f64 * t24646 - t11925 * t2155 + t24758 * t498 - t1238 * t24868 - t7351 * t3631 + t24871 * t498 + 2.0_f64 * t24873 * t498 + 2.0_f64 * t1238 * t24877 - 2.0_f64 * t24880 * t1252 - 0.27415567780803773942e-2_f64 * t7283 * t24884 - 0.54831135561607547884e-2_f64 * t7283 * t24888 - 0.18277045187202515961e-2_f64 * t24891 - 2.0_f64 * t24893 * t1252 - 6.0_f64 * t1238 * t24897;
    (t24883, t24884, t24887, t24888, t24891, t24893, t24897, t24900)
}
