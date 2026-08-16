//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1361/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1361(t72044: f64, t72057: f64, t72069: f64, t72077: f64, t4715: f64, t5831: f64, t1395: f64, t14423: f64, t17993: f64, t18000: f64, t18006: f64, t18021: f64, t1805: f64, t19736: f64, t19762: f64, t19769: f64, t20449: f64, t20482: f64, t20503: f64, t21608: f64, t2162: f64, t21623: f64, t21624: f64, t21630: f64, t21634: f64, t21635: f64, t21645: f64, t21653: f64, t226: f64, t253: f64, t3699: f64, t3721: f64, t5571: f64, t5572: f64, t5577: f64, t5843: f64, t61195: f64, t61226: f64, t6342: f64, t6343: f64, t6348: f64, t64135: f64, t66480: f64, t69912: f64, t782: f64, t818: f64, param_beta: f64) -> (f64, f64) {
    let t72079 = t72044 + t72057 + t72069 + t72077;
    let t72111 = t5831 * t4715;
    let t72129 = 24.0_f64 * t5571 * t61195 * t21623 * t818 + 2.0_f64 * t5571 * t5572 * t1805 * t14423 + param_beta * t72079 * t253 + 2.0_f64 * t17993 * t21635 - 6.0_f64 * t5571 * t18000 * t21634 * t818 + 2.0_f64 * t64135 * t6348 - 6.0_f64 * t17993 * t21624 + 8.0_f64 * t18006 * t20482 * t1395 * t19769 - 12.0_f64 * t5571 * t18000 * t21630 * t818 - 12.0_f64 * t5571 * t18000 * t6342 * t3721 + 2.0_f64 * t19736 * t20503 + t5571 * t5577 * t21608 * t782 * t226 + t17993 * t21653 + t5571 * t5577 * t72111 * t226 - 2.0_f64 * t5571 * t18021 * t72111 * t2162 + 2.0_f64 * t17993 * t21645 + 4.0_f64 * t64135 * t6343 + 12.0_f64 * t61226 * t66480 * t19762 + t69912 * t5843 + 4.0_f64 * t20449 * t3699;
    (t72079, t72129)
}
