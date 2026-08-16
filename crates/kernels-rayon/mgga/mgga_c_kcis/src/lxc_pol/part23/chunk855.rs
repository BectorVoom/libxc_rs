//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 855/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk855(t3873: f64, t5481: f64, t1319: f64, t3809: f64, t5556: f64, t11402: f64, t1897: f64, t3781: f64, t16048: f64, t16050: f64, t11408: f64, t11409: f64, t11411: f64, t11413: f64, t11415: f64, t16046: f64, t16052: f64, t16057: f64, t16062: f64, t16067: f64, t16071: f64, t16075: f64, t16080: f64, t16084: f64, t16088: f64) -> (f64, f64, f64, f64) {
    let t16162 = t3873 * t5481;
    let t16163 = t16162 * t1319;
    let t16165 = t5556 * t3809;
    let t16167 = t11402 * t1897;
    let t16168 = t16167 * t3781;
    let t16183 = 4.0_f64 / 27.0_f64 * t16048;
    let t16184 = 4.0_f64 / 9.0_f64 * t16050;
    let t16194 = -t11408 - 8.0_f64 / 27.0_f64 * t11409 + 2.0_f64 / 27.0_f64 * t11411 - 2.0_f64 / 9.0_f64 * t11413 + t11415 / 9.0_f64 - 4.0_f64 / 27.0_f64 * t16046 + t16183 - t16184 - 22.0_f64 / 9.0_f64 * t16052 - 10.0_f64 / 27.0_f64 * t16057 + 4.0_f64 / 3.0_f64 * t16062 + 8.0_f64 / 9.0_f64 * t16067 - 2.0_f64 / 9.0_f64 * t16071 - 2.0_f64 * t16075 - 8.0_f64 / 3.0_f64 * t16080 + 2.0_f64 / 3.0_f64 * t16084 + 2.0_f64 / 3.0_f64 * t16088;
    (t16163, t16165, t16168, t16194)
}
