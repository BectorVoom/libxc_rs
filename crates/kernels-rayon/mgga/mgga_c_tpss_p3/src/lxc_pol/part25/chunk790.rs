//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 790/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk790(t5352: f64, t5462: f64, t118: f64, t1322: f64, t1339: f64, t1600: f64, t1604: f64, t1663: f64, t3493: f64, t4631: f64, t4638: f64, t4641: f64, t4675: f64, t485: f64, t488: f64, t5314: f64, t5322: f64, t544: f64, t626: f64) -> (f64, f64) {
    let t5463 = t5352 + t5462;
    let t5465 = -t118 * t5314 - 2.0_f64 * t1322 * t1600 - 4.0_f64 * t1339 * t3493 + 2.0_f64 * t1604 * t1663 - t4631 * t485 - 2.0_f64 * t4638 * t485 - 4.0_f64 * t4641 * t626 - 2.0_f64 * t4675 * t626 + t488 * t5463 + t5322 * t544;
    (t5463, t5465)
}
