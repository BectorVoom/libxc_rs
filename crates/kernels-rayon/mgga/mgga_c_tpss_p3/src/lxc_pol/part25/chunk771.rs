//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 771/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk771(t1139: f64, t5294: f64, t1136: f64, t1587: f64, t4296: f64, t473: f64, t5271: f64, t5276: f64, t1589: f64, t1153: f64, t198: f64, t3154: f64, t330: f64, t5078: f64, t5080: f64, t5084: f64, t5116: f64, t5119: f64, t5185: f64, t5187: f64, t5189: f64, t5193: f64, t5197: f64, t5201: f64) -> (f64, f64, f64, f64) {
    let t5295 = t1139 * t5294;
    let t5297 = 2.0_f64 * t1136 * t5276 - t1136 * t5295 - 2.0_f64 * t1587 * t4296 + t473 * t5271;
    let t5301 = t1589 * t1589;
    let t5305 = t1153 * t198 * t330 * t5297 - t198 * t3154 * t330 * t5301 - t5078 + t5080 - t5084 + t5116 + t5119 + t5185 + t5187 - t5189 + t5193 - t5197 - t5201;
    (t5295, t5297, t5301, t5305)
}
