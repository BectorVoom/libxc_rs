//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1324/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1324(t3628: f64, t4186: f64, t5351: f64, t3626: f64, t12910: f64, t17283: f64, t17375: f64, t17448: f64, t17605: f64, t1791: f64, t21001: f64, t21004: f64, t21008: f64, t21014: f64, t21017: f64, t3625: f64, t5320: f64, t5323: f64, t5335: f64, t5343: f64, t5402: f64, t5407: f64) -> f64 {
    let t21020 = t3628 * t4186;
    let t21021 = t5351 * t21020;
    let t21022 = t3626 * t21021;
    let t21027 = 0.22866142996303859718e-2_f64 * t17283 * t1791 + 0.22866142996303859718e-2_f64 * t5323 * t5320 - 0.28582678745379824648e-3_f64 * t17448 * t5407 + t17375 + 0.10162730220579493208e-2_f64 * t21001 + 0.85748036236139473944e-3_f64 * t12910 * t21004 + 0.23818898954483187207e-3_f64 * t3625 * t21008 + 0.15244095330869239812e-2_f64 * t17605 * t5407 - 0.45732285992607719436e-2_f64 * t21014 * t5343 + 0.22866142996303859718e-2_f64 * t21017 * t5335 - 0.28582678745379824648e-3_f64 * t3625 * t21022 - 0.28582678745379824648e-3_f64 * t17448 * t5402;
    t21027
}
