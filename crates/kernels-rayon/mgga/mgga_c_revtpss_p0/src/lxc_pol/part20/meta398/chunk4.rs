//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1477/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1477(t11404: f64, t11409: f64, t11444: f64, t11450: f64, t11517: f64, t11521: f64, t11551: f64, t11554: f64, t2943: f64, t2944: f64, t2968: f64, t2970: f64, t311: f64, t41540: f64, t41668: f64, t41763: f64, t41864: f64, t41867: f64, t41871: f64, t41873: f64, t41876: f64, t41879: f64, t41882: f64, t41885: f64, t41888: f64, t41895: f64, t41913: f64, t41926: f64, t953: f64, t954: f64) -> f64 {
    let t41930 = 24.0_f64 * t11404 * t11551 - 24.0_f64 * t11409 * t41668 * t954 - 6.0_f64 * t2943 * t41763 * t954 + t41864 + t41867 - t41871 - t41873 + t41876 + t41879 + t41882 + t41885 - t41888 + 0.3859675079686208416e3_f64 * t11404 * t11517 + 0.12865583598954028054e3_f64 * t2968 * t11444 * t2970 * t953 + 0.12414243100625616072e5_f64 * t11450 * t41895 * t2944 - 0.14035736694323150897e2_f64 * t11554 * t11521 - 0.19751673498613801407e-1_f64 * t41540 - 0.310907e-1_f64 * (t41913 + t41926) * t311;
    t41930
}
