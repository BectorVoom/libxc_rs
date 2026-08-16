//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 487/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk487(t2327: f64, t598: f64, t2022: f64, t2024: f64, t2039: f64, t2078: f64, t2084: f64, t2098: f64, t2102: f64, t2291: f64, t2295: f64, t2300: f64, t2305: f64, t2307: f64, t2311: f64, t2315: f64, t2319: f64, t2321: f64, t2323: f64) -> f64 {
    let t2328 = t598 * t2327;
    let t2330 = -t2022 - t2024 + 0.47172138434406228102e-3_f64 * t2291 + t2039 - 0.21437009059034868486e-3_f64 * t2295 + 0.7862023072401038017e-3_f64 * t2300 - 0.31448092289604152068e-3_f64 * t2305 - 0.42874018118069736972e-3_f64 * t2307 - 0.53592522647587171215e-3_f64 * t2311 + 0.114609375e-1_f64 * t2315 + 0.7640625e-2_f64 * t2319 + 0.85748036236139473944e-3_f64 * t2321 - 0.85748036236139473944e-3_f64 * t2323 + 0.10718504529517434243e-3_f64 * t2328 + t2078 - t2084 - t2098 + t2102;
    t2330
}
