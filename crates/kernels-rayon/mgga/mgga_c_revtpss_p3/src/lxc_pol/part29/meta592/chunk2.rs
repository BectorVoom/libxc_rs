//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1969/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1969(t2435: f64, t8099: f64, t25904: f64, t26231: f64, t97802: f64, t26234: f64, t98041: f64, t102244: f64, t94674: f64, t97700: f64, t102268: f64, t1882: f64, t25921: f64, t25930: f64, t26335: f64, t28863: f64, t28890: f64, t28911: f64, t7292: f64, t7917: f64, t96296: f64, t96298: f64, t96371: f64, t96374: f64, t96378: f64, t98362: f64) -> (f64, f64) {
    let t102315 = t8099 * t2435;
    let t102316 = t25904 * t102315;
    let t102320 = 0.14456046980341999104e-1_f64 * t97802 * t26231;
    let t102324 = 0.51405703062096148812e-1_f64 * t98041 * t26234;
    let t102325 = t94674 * t102244;
    let t102329 = 0.28912093960683998208e-1_f64 * t97700 * t26234;
    let t102339 = 0.14456046980341999104e-1_f64 * t25904 * t102268;
    let t102341 = -0.28912093960683998208e-1_f64 * t96296 + 0.96373646535613327357e-2_f64 * t102316 + 0.19274729307122665471e-1_f64 * t96298 - t102320 - 0.4336814094102599731e0_f64 * t7917 * t26335 + t102324 + 0.86736281882051994623e-1_f64 * t102325 - 0.12851425765524037203e-1_f64 * t96371 + t96374 - t102329 + 0.34694512752820797848e1_f64 * t25930 * t28911 * t1882 * t98362 + 0.17347256376410398924e1_f64 * t25921 * t28863 - 0.8673628188205199462e0_f64 * t7292 * t28890 - t102339 - 0.77108554593144223218e-1_f64 * t96378;
    (t102315, t102341)
}
