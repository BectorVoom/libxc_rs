//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3685/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3685(t17376: f64, t17528: f64, t3140: f64, t6564: f64, t3599: f64, t1042: f64, t1261: f64, t17199: f64, t17204: f64, t17235: f64, t17558: f64, t21107: f64, t3591: f64, t3606: f64, t3613: f64, t5279: f64, t5302: f64, t5381: f64, t5391: f64, t58927: f64, t60834: f64, t65829: f64, t69661: f64, t69668: f64, t69674: f64, t69680: f64) -> (f64, f64) {
    let t69683 = t17376 * t17528;
    let t69692 = t6564 * t3140;
    let t69693 = t69692 * t3599;
    let t69696 = 0.31758531939310916276e-4_f64 * t69661 + 0.23818898954483187207e-3_f64 * t1261 * t1042 * t5302 * t60834 - 0.47637797908966374413e-4_f64 * t69668 - 0.57165357490759649296e-3_f64 * t5381 * t17199 - 0.17149607247227894789e-2_f64 * t5381 * t17204 + 0.96545937095505185476e-2_f64 * t69674 - 0.30488190661738479624e-2_f64 * t58927 * t5279 - 0.22866142996303859718e-2_f64 * t21107 * t3591 - 0.45732285992607719436e-2_f64 * t69680 * t3606 + 0.22866142996303859718e-2_f64 * t69683 * t3613 - 0.63517063878621832552e-3_f64 * t1261 * t1042 * t17235 * t65829 - 0.2540682555144873302e-2_f64 * t5391 * t17558 + 0.42874018118069736972e-3_f64 * t69693 * t3606;
    (t69692, t69696)
}
