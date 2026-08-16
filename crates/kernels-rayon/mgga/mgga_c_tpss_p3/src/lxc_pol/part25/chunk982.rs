//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 982/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk982(t13546: f64, t485: f64, t1163: f64, t1322: f64, t13452: f64, t13458: f64, t13463: f64, t13470: f64, t13473: f64, t13478: f64, t1600: f64, t2056: f64, t3491: f64, t3499: f64, t4341: f64, t4631: f64, t4638: f64, t4641: f64, t4675: f64, t5314: f64, t624: f64, t626: f64) -> (f64, f64) {
    let t13547 = t485 * t13546;
    let t13551 = -t1163 * t4631 - 2.0_f64 * t1163 * t4638 - 2.0_f64 * t1322 * t4341 - t13452 * t485 - 2.0_f64 * t13458 * t485 - 2.0_f64 * t13463 * t626 - 2.0_f64 * t13470 * t626 - 4.0_f64 * t13473 * t626 - 4.0_f64 * t13478 * t626 - 2.0_f64 * t13547 * t626 - 2.0_f64 * t1600 * t3491 - 2.0_f64 * t2056 * t4675 - 4.0_f64 * t3499 * t4641 - 2.0_f64 * t3499 * t4675 - t5314 * t624;
    (t13547, t13551)
}
