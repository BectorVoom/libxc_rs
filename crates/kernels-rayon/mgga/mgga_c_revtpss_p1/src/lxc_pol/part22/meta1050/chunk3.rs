//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3698/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3698(t17306: f64, t17728: f64, t489: f64, t1261: f64, t12879: f64, t247: f64, t6425: f64, t12772: f64, t21227: f64, t3625: f64, t21021: f64, t12855: f64, t13396: f64, t16714: f64, t17254: f64, t17461: f64, t17739: f64, t20795: f64, t21143: f64, t21203: f64, t3584: f64, t3603: f64, t3620: f64, t3626: f64, t3720: f64, t44225: f64, t5402: f64, t56853: f64, t56867: f64, t57005: f64, t57275: f64, t57571: f64, t59411: f64, t6638: f64) -> f64 {
    let t70014 = t17306 * t489 * t17728;
    let t70032 = t1261 * t247 * t12879 * t6425;
    let t70039 = t3625 * t12772 * t21227;
    let t70044 = t3625 * t12772 * t21021;
    let t70050 = -0.11433071498151929859e-2_f64 * t70014 * t17739 - 0.2540682555144873302e-2_f64 * t57005 * t44225 * t16714 * t13396 - 0.42874018118069736972e-3_f64 * t12855 * t3720 * t20795 * t3603 * t3584 + 0.23818898954483187207e-3_f64 * t21143 * t3620 - 0.91464571985215438872e-2_f64 * t21203 * t17254 + 0.6351706387862183255e-4_f64 * t70032 + 0.17149607247227894789e-2_f64 * t59411 * t17461 + 0.3811023832717309953e-3_f64 * t56853 - 0.76220476654346199061e-3_f64 * t56867 - 0.3811023832717309953e-3_f64 * t70039 + 0.30488190661738479624e-2_f64 * t57571 * t5402 - 0.3811023832717309953e-3_f64 * t70044 - 0.28582678745379824648e-3_f64 * t3625 * t3626 * t57275 * t6638;
    t70050
}
