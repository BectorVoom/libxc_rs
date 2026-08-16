//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3754/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3754(t12787: f64, t13392: f64, t17244: f64, t17351: f64, t17396: f64, t17602: f64, t17672: f64, t17729: f64, t20770: f64, t20921: f64, t3604: f64, t3625: f64, t3720: f64, t44551: f64, t44902: f64, t44906: f64, t44917: f64, t5373: f64, t56981: f64, t59094: f64, t6421: f64, t71460: f64, t71470: f64, t71476: f64, t71480: f64, t71490: f64) -> f64 {
    let t71492 = 0.31758531939310916275e-4_f64 * t44902 + 0.6351706387862183255e-4_f64 * t44906 + t71460 / 81.0_f64 - 0.57165357490759649296e-3_f64 * t59094 + 2.0_f64 / 27.0_f64 * t5373 * t17244 + 0.6351706387862183255e-4_f64 * t44917 + 0.57165357490759649296e-3_f64 * t17351 * t56981 * t20770 + 0.3811023832717309953e-3_f64 * t71470 - 0.47637797908966374413e-3_f64 * t17729 * t12787 * t20921 * t13392 + 0.30488190661738479624e-2_f64 * t71476 + 0.22866142996303859718e-2_f64 * t17396 * t17602 + 0.85748036236139473944e-3_f64 * t44551 * t3720 * t71480 * t3604 + 0.23818898954483187207e-3_f64 * t3625 * t12787 * t6421 * t17672 - 0.17149607247227894789e-2_f64 * t71490;
    t71492
}
