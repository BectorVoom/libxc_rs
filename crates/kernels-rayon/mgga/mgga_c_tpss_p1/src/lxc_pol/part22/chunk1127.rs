//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1127/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1127(t11453: f64, t4284: f64, t1125: f64, t12431: f64, t12435: f64, t12439: f64, t12443: f64, t12446: f64, t12448: f64, t12451: f64, t12455: f64, t12460: f64, t12465: f64, t12467: f64, t12472: f64, t12477: f64, t3052: f64, t3057: f64, t3070: f64, t3076: f64, t3080: f64, t3083: f64, t4258: f64, t9607: f64, t9664: f64, t9669: f64, t9673: f64, t9677: f64, t9701: f64) -> f64 {
    let t12478 = t11453 * t4284;
    let t12480 = t1125 * t12478 / 1728.0_f64;
    let t12481 = -t9664 / 432.0_f64 + t9669 / 10368.0_f64 - t9673 / 6912.0_f64 - t9677 / 3456.0_f64 - t4258 * t3076 / 576.0_f64 - t12431 * t3057 / 288.0_f64 + t12435 * t3083 / 576.0_f64 + t12439 + t12443 + t9701 - t12446 / 13824.0_f64 + t12448 / 2592.0_f64 - t3080 * t12451 / 3072.0_f64 + t9607 * t12455 / 3072.0_f64 - t3080 * t12460 / 1536.0_f64 - t12465 + t3052 * t12467 / 768.0_f64 + t12472 * t3070 / 432.0_f64 - t12477 - t12480;
    t12481
}
