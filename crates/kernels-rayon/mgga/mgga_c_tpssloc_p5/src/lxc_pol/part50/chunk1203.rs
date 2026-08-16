//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1203/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1203(t23665: f64, t32935: f64, t113508: f64, t113511: f64, t113526: f64, t1920: f64, t1948: f64, t23327: f64, t23346: f64, t23613: f64, t23670: f64, t25499: f64, t25523: f64, t25567: f64, t25705: f64, t30885: f64, t3186: f64, t32927: f64, t32928: f64, t32943: f64, t345: f64, t4673: f64, t6797: f64, t6799: f64, t6800: f64) -> f64 {
    let t119221 = t23665 * t32935;
    let t119232 = -0.16449340668482264365e-1_f64 * t6797 * t25523 * t30885 + 0.16449340668482264365e-1_f64 * t1920 * t345 * t1948 * t25705 - 0.54831135561607547883e-2_f64 * t113508 + 2.0_f64 * t3186 * t32943 * t4673 + 0.54831135561607547883e-2_f64 * t113511 + 0.16449340668482264365e-1_f64 * t6797 * t6799 * t25567 * t6800 - 0.54831135561607547883e-2_f64 * t23327 * t23613 * t32927 + 0.54831135561607547883e-2_f64 * t119221 + 0.16449340668482264365e-1_f64 * t6797 * t6799 * t25499 * t6800 - 0.43864908449286038307e-1_f64 * t23670 * t32935 + 0.54831135561607547883e-2_f64 * t113526 - 0.14621636149762012769e-1_f64 * t23346 * t32928;
    t119232
}
