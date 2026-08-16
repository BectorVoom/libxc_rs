//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1413/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1413(t2832: f64, t11054: f64, t892: f64, t11084: f64, t1940: f64, t198: f64, t207: f64, t2394: f64, t2403: f64, t2411: f64, t40076: f64, t40079: f64, t40190: f64, t40194: f64, t40198: f64, t40202: f64, t40204: f64, t40206: f64, t40209: f64, t40212: f64, t4541: f64, t775: f64, t890: f64) -> f64 {
    let t41192 = t2832 * t2832;
    let t41197 = t11054 * t892;
    let t41208 = -4.0_f64 * t11054 * t1940 * t2411 * t890 - 3.0_f64 * t198 * t207 * t2411 * t41192 - 36.0_f64 * t11084 * t2394 * t4541 + 12.0_f64 * t2403 * t41197 * t775 + t40076 - t40079 + t40190 + t40194 + t40198 + t40202 + t40204 - t40206 + t40209 + t40212;
    t41208
}
