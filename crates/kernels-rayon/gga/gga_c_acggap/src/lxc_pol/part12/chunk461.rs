//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 461/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk461(t2023: f64, t2038: f64, t1974: f64, t2002: f64, t2004: f64, t2006: f64, t2026: f64, t2033: f64, t2179: f64, t2180: f64, t2182: f64, t2183: f64, t2184: f64, t2185: f64, t2189: f64, t2190: f64, t2191: f64, t2192: f64) -> (f64, f64, f64) {
    let t2193 = 11.0_f64 / 576.0_f64 * t2023;
    let t2196 = t2038 / 96.0_f64;
    let t2197 = t2179 - t2180 + 0.21437009059034868486e-3_f64 * t1974 + t2182 - t2183 - t2184 - t2185 - 0.34299214494455789578e-2_f64 * t2002 + 0.17149607247227894789e-2_f64 * t2004 - 0.17149607247227894789e-2_f64 * t2006 - t2189 + t2190 + t2191 - t2192 - t2193 + t2026 / 48.0_f64 + 0.22921875e-1_f64 * t2033 + t2196;
    (t2193, t2196, t2197)
}
