//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3717/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3717(t3666: f64, t6594: f64, t17283: f64, t5362: f64, t1238: f64, t12832: f64, t17280: f64, t17405: f64, t17672: f64, t1791: f64, t20851: f64, t21042: f64, t21177: f64, t3625: f64, t3626: f64, t3663: f64, t5320: f64, t5323: f64, t5373: f64, t57173: f64, t57176: f64, t57178: f64, t59025: f64, t6429: f64) -> f64 {
    let t70469 = t3666 * t6594;
    let t70476 = t17283 * t5362;
    let t70480 = -0.14291339372689912324e-3_f64 * t3625 * t3626 * t6429 * t17672 + 0.11433071498151929859e-2_f64 * t57173 - 0.3811023832717309953e-3_f64 * t57176 + 0.28582678745379824648e-3_f64 * t57178 - 0.42874018118069736972e-3_f64 * t12832 * t21042 + 0.22866142996303859718e-2_f64 * t59025 * t1791 + 0.45732285992607719436e-2_f64 * t17283 * t5320 + 0.22866142996303859718e-2_f64 * t5323 * t17280 - 0.14481890564325777821e-1_f64 * t70469 * t1238 - 0.72409452821628889107e-2_f64 * t21177 * t3663 - 0.21437009059034868486e-3_f64 * t20851 * t3663 + 0.30488190661738479624e-2_f64 * t70476 + t5373 * t17405 / 54.0_f64;
    t70480
}
