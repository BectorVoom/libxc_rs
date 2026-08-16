//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1090/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1090(t15043: f64, t2741: f64, t4826: f64, t949: f64, t8523: f64, t361: f64, t4988: f64, t3933: f64, t3931: f64, t11586: f64, t11590: f64, t14999: f64, t15002: f64, t15005: f64, t15012: f64, t15018: f64, t15021: f64, t15028: f64, t15032: f64, t15036: f64, t15040: f64, t2722: f64, t2740: f64, t2748: f64, t4980: f64, t5001: f64, t5009: f64, t8509: f64, t8972: f64, t9033: f64, t9038: f64, t925: f64, t967: f64) -> (f64, f64) {
    let t15044 = t2741 * t15043;
    let t15047 = t4826 * t949;
    let t15048 = t8523 * t15047;
    let t15051 = t361 * t4988;
    let t15052 = t15051 * t3933;
    let t15053 = t3931 * t15052;
    let t15056 = -t925 * t14999 / 72.0_f64 - t925 * t15002 / 144.0_f64 + t925 * t15005 / 216.0_f64 - 5.0_f64 / 2592.0_f64 * t2748 * t5001 - t15012 / 3456.0_f64 - t2748 * t5009 / 864.0_f64 + t15018 / 6912.0_f64 + t967 * t15021 / 4608.0_f64 + t11586 - t8972 * t4980 / 288.0_f64 + t15028 / 2304.0_f64 + t11590 + t9033 / 2592.0_f64 + t9038 + t2740 * t15032 / 2304.0_f64 + t2740 * t15036 / 2304.0_f64 + t2740 * t15040 / 4608.0_f64 - t8509 * t15044 / 4608.0_f64 + 5.0_f64 / 13824.0_f64 * t2740 * t15048 + t2722 * t15053 / 1536.0_f64;
    (t15051, t15056)
}
