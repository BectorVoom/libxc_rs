//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 711/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk711(t1300: f64, t1303: f64, t2024: f64, t44: f64, t4589: f64, t4592: f64, t4597: f64, t4602: f64, t4605: f64, t56: f64, t61: f64, t38: f64) -> (f64, f64) {
    let t4608 = 5.0_f64 / 18.0_f64 * t44 * t4589 + 5.0_f64 / 6.0_f64 * t44 * t4592 + 88.0_f64 / 9.0_f64 * t4597 * t61 + 40.0_f64 / 9.0_f64 * t1300 * t1303 + 5.0_f64 / 18.0_f64 * t56 * t4602 - 5.0_f64 / 6.0_f64 * t56 * t4605 - t2024;
    let t4609 = t38 * t4608;
    (t4608, t4609)
}
