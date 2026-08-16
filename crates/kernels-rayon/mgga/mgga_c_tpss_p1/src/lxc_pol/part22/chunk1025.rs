//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1025/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1025(t10980: f64, t10986: f64, t11003: f64, t11005: f64, t11006: f64, t11010: f64, t11015: f64, t11020: f64, t11024: f64, t11028: f64, t11033: f64, t11037: f64, t8605: f64, t8607: f64, t8616: f64, t8618: f64, t8687: f64) -> f64 {
    let t11040 = -t8687 - 8.0_f64 / 27.0_f64 * t8616 + 2.0_f64 / 27.0_f64 * t8607 - 2.0_f64 / 9.0_f64 * t8618 + t8605 / 9.0_f64 - 4.0_f64 / 27.0_f64 * t10980 + t11003 - t11005 + t11006 - 10.0_f64 / 27.0_f64 * t11010 + 4.0_f64 / 3.0_f64 * t11015 - 4.0_f64 / 9.0_f64 * t11020 - 2.0_f64 / 9.0_f64 * t11024 - 2.0_f64 * t11028 + 4.0_f64 / 3.0_f64 * t11033 + 2.0_f64 / 3.0_f64 * t11037 - t10986 / 3.0_f64;
    t11040
}
