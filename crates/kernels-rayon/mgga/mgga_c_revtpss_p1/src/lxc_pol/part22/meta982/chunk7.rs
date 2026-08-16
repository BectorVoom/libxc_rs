//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3330/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3330(t1544: f64, t2411: f64, t14365: f64, t1583: f64, t18392: f64, t18865: f64, t1940: f64, t198: f64, t205: f64, t2403: f64, t2404: f64, t2408: f64, t2832: f64, t40076: f64, t40079: f64, t40194: f64, t40198: f64, t41154: f64, t6079: f64, t61519: f64, t62307: f64, t62308: f64, t62311: f64, t62312: f64, t765: f64) -> f64 {
    let t63185 = t2411 * t1544;
    let t63186 = t63185 * t14365;
    let t63189 = -24.0_f64 * t1583 * t198 * t205 * t63186 - 6.0_f64 * t1940 * t2408 * t41154 * t6079 + 6.0_f64 * t18392 * t2403 * t2404 - t18865 * t1940 * t2832 + 3.0_f64 * t198 * t61519 * t765 + t40076 - t40079 + t40194 + t40198 + t62307 - t62308 + t62311 - t62312;
    t63189
}
