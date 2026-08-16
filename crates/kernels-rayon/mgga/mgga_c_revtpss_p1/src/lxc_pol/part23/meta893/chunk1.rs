//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2850/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2850(t61114: f64, t18569: f64, t4311: f64, t22671: f64, t706: f64, t750: f64, t39483: f64, t39520: f64, t39528: f64, t39531: f64, t39534: f64, t39537: f64, t39540: f64, t76944: f64, t76946: f64, t76948: f64, t76950: f64, t76951: f64, t76952: f64, t76954: f64) -> (f64, f64, f64, f64) {
    let t76955 = 3.0_f64 * t61114;
    let t76957 = 12.0_f64 * t4311 * t18569;
    let t76959 = t706 * t750 * t22671;
    let t76960 = 4.0_f64 * t76959;
    let t76961 = -t76944 + t76946 + t76948 + t76950 + t76951 - t76952 - t39483 + t76954 + t39520 + t76955 - t39528 + t76957 + t39531 + t76960 + t39534 + t39537 - t39540;
    (t76955, t76957, t76960, t76961)
}
