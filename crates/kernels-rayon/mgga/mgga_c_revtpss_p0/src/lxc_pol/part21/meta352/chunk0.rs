//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1693/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1693(t11735: f64, t345: f64, t10345: f64, t344: f64, t247: f64, t2858: f64, t3109: f64, t1063: f64, t1066: f64, t11160: f64, t1068: f64, t11707: f64, t11712: f64, t11714: f64, t11723: f64, t11728: f64, t11730: f64, t11732: f64, t3091: f64, t3101: f64, t3106: f64, t3177: f64, t3184: f64, t348: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11737 = 5.0_f64 / 1296.0_f64 * t345 * t11735;
    let t11738 = t10345 * t344;
    let t11744 = t247 * t3109 * t2858;
    let t11745 = t1063 * t11744;
    let t11748 = t247 * t1066 * t11160;
    let t11751 = 0.7145669686344956162e-3_f64 * t3091 * t11707 + 0.57165357490759649295e-3_f64 * t11712 - 0.45732285992607719436e-2_f64 * t11714 * t1068 - 0.22866142996303859718e-2_f64 * t3106 * t3177 - 0.3811023832717309953e-2_f64 * t3106 * t3184 + 0.28582678745379824648e-3_f64 * t11723 + 0.47637797908966374413e-3_f64 * t11728 + 11.0_f64 / 108.0_f64 * t11730 + t11732 / 54.0_f64 + t11737 - 77.0_f64 / 162.0_f64 * t11738 * t348 + 0.45732285992607719436e-2_f64 * t3106 * t3101 - 0.57165357490759649295e-3_f64 * t11745 + 0.85748036236139473944e-3_f64 * t1063 * t11748;
    (t11737, t11738, t11744, t11745, t11748, t11751)
}
