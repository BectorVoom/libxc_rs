//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2154/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2154(t1058: f64, t29779: f64, t100146: f64, t100240: f64, t100261: f64, t100262: f64, t100268: f64, t100270: f64, t100272: f64, t1972: f64, t19857: f64, t25539: f64, t375: f64, t4783: f64, t6285: f64, t6317: f64, t7125: f64) -> f64 {
    let t107107 = t29779 * t1058;
    let t107120 = 0.57165357490759649296e-3_f64 * t100146 * t4783 + t100240 + t100261 - 0.7622047665434619906e-3_f64 * t100262 + 0.28582678745379824648e-3_f64 * t107107 - 0.22866142996303859718e-2_f64 * t6317 * t7125 * t375 + 0.42874018118069736972e-3_f64 * t19857 * t1972 * t375 + t25539 * t6285 / 54.0_f64 + 0.38110238327173099531e-3_f64 * t100268 - 0.30488190661738479625e-2_f64 * t100270 - 0.19055119163586549765e-3_f64 * t100272;
    t107120
}
