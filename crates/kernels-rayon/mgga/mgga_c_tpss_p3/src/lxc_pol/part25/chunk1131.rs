//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1131/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1131(t1098: f64, t1103: f64, t12361: f64, t12368: f64, t12371: f64, t12385: f64, t15544: f64, t15547: f64, t15550: f64, t15554: f64, t15558: f64, t15561: f64, t15564: f64) -> f64 {
    let t15566 = t1098 * t15544 / 108.0_f64 - t1098 * t15547 / 72.0_f64 - t1098 * t15550 / 48.0_f64 - t12361 + t12368 / 10368.0_f64 - t12371 - 11.0_f64 / 324.0_f64 * t15554 * t1103 - t15558 / 432.0_f64 + t15561 / 648.0_f64 + t12385 / 648.0_f64 + 11.0_f64 / 324.0_f64 * t15564;
    t15566
}
