//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2711/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2711(t190: f64, t49889: f64, t706: f64, t4398: f64, t9387: f64, t11061: f64, t15071: f64, t1583: f64, t1940: f64, t2411: f64, t39442: f64, t41154: f64, t49872: f64, t49873: f64, t49877: f64, t49879: f64, t49882: f64, t49885: f64, t890: f64) -> (f64, f64, f64) {
    let t49892 = 4.0_f64 * t706 * t190 * t49889;
    let t49897 = t4398 * t9387;
    let t49898 = 0.5848223622634646207e0_f64 * t49897;
    let t49903 = -6.0_f64 * t11061 * t1583 * t1940 * t41154 - 3.0_f64 * t15071 * t1940 * t2411 * t890 + t39442 + t49872 + t49873 + t49877 + t49879 + t49882 + t49885 + t49892 - t49898;
    (t49892, t49898, t49903)
}
