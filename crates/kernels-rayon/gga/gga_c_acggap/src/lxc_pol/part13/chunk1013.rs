//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1013/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1013(t30226: f64, t30240: f64, t5213: f64, t7822: f64, t157: f64, t33750: f64, t1165: f64, t2068: f64, t604: f64, t30230: f64, t30233: f64, t30234: f64, t30239: f64, t30243: f64, t30247: f64, t30249: f64, t30251: f64, t30253: f64, t33956: f64, t33960: f64, t33963: f64, t33966: f64) -> (f64, f64) {
    let t33968 = 0.17149607247227894789e-2_f64 * t30226;
    let t33970 = 0.21437009059034868486e-3_f64 * t30240;
    let t33974 = t7822 * t5213;
    let t33976 = t33750 * t157;
    let t33979 = t2068 * t1165 * t604 * t33976;
    let t33981 = -0.21437009059034868486e-2_f64 * t33956 - 0.38203125e-2_f64 * t33960 + t33963 - t33966 / 128.0_f64 + t33968 + t30230 + t30233 - 0.85748036236139473944e-3_f64 * t30234 + t30239 + t33970 + t30243 - t30247 - 0.90702367218671976886e-1_f64 * t30249 - 0.12004725073059526352e-1_f64 * t30251 + 0.85748036236139473945e-2_f64 * t30253 - 0.17149607247227894789e-2_f64 * t33974 + 0.15724046144802076034e-3_f64 * t33979;
    (t33976, t33981)
}
