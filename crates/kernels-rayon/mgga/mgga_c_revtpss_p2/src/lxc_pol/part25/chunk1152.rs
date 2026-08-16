//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1152/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1152(t1971: f64, t3104: f64, t351: f64, t25516: f64, t3114: f64, t1068: f64, t25543: f64, t25551: f64, t25554: f64, t25557: f64, t25560: f64, t25561: f64, t25564: f64, t25566: f64, t25569: f64, t3101: f64, t3120: f64, t3177: f64, t3184: f64, t3238: f64, t3248: f64, t3255: f64, t375: f64, t7111: f64, t7132: f64) -> (f64, f64, f64, f64) {
    let t25576 = t1971 * t3104;
    let t25577 = t351 * t25576;
    let t25580 = t3114 * t25516;
    let t25585 = t25543 / 432.0_f64 + t7111 * t3248 / 288.0_f64 + t7111 * t3255 / 216.0_f64 - t7111 * t3238 / 144.0_f64 + 0.3811023832717309953e-3_f64 * t25551 + 0.14481890564325777821e-1_f64 * t25554 * t375 - 0.30488190661738479624e-2_f64 * t25557 - t25560 - 0.45732285992607719436e-2_f64 * t25561 * t375 + 0.57165357490759649296e-3_f64 * t25564 + 0.42874018118069736972e-3_f64 * t25566 * t375 + 0.57165357490759649296e-3_f64 * t25569 * t1068 + 0.47637797908966374413e-3_f64 * t7132 * t3184 + 0.28582678745379824648e-3_f64 * t7132 * t3177 - 0.30488190661738479624e-2_f64 * t25577 * t1068 - 0.85748036236139473944e-3_f64 * t25580 * t3120 - 0.57165357490759649296e-3_f64 * t7132 * t3101;
    (t25576, t25577, t25580, t25585)
}
