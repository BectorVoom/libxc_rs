//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 481/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk481(t2192: f64, t2193: f64, t2196: f64, t2206: f64, t2207: f64, t2210: f64, t2211: f64, t2291: f64, t2295: f64, t2300: f64, t2305: f64, t2307: f64, t2311: f64, t2315: f64, t2319: f64, t2321: f64, t2323: f64, t2328: f64) -> f64 {
    let t2384 = -t2192 - t2193 + 0.94344276868812456207e-3_f64 * t2291 + t2196 - 0.42874018118069736972e-3_f64 * t2295 + 0.15724046144802076034e-2_f64 * t2300 - 0.62896184579208304138e-3_f64 * t2305 - 0.85748036236139473944e-3_f64 * t2307 - 0.10718504529517434243e-2_f64 * t2311 + 0.22921875e-1_f64 * t2315 + 0.1528125e-1_f64 * t2319 + 0.17149607247227894789e-2_f64 * t2321 - 0.17149607247227894789e-2_f64 * t2323 + 0.21437009059034868486e-3_f64 * t2328 + t2206 - t2207 - t2210 + t2211;
    t2384
}
