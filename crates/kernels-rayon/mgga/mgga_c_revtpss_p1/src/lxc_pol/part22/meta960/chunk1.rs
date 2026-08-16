//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3222/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3222(t18281: f64, t706: f64, t750: f64, t39737: f64, t190: f64, t60754: f64, t18838: f64, t892: f64, t11075: f64, t14375: f64, t18435: f64, t198: f64, t2403: f64, t2404: f64, t39540: f64, t39741: f64, t39744: f64, t39747: f64, t39750: f64, t39756: f64, t4541: f64, t5962: f64, t775: f64) -> (f64, f64, f64, f64) {
    let t61130 = t706 * t750 * t18281;
    let t61131 = 8.0_f64 * t61130;
    let t61135 = 8.0_f64 * t39737;
    let t61138 = 4.0_f64 * t706 * t190 * t60754;
    let t61139 = t18838 * t892;
    let t61146 = 3.0_f64 * t11075 * t2403 * t5962 + 6.0_f64 * t14375 * t198 * t5962 + 12.0_f64 * t18435 * t2404 * t4541 + 6.0_f64 * t2403 * t61139 * t775 - t39540 + t39741 + t39744 + t39747 + t39750 + t39756 + t61131 + t61135 + t61138;
    (t61131, t61135, t61138, t61146)
}
