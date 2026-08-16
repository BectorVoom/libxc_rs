//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 676/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk676(t1113: f64, t2785: f64, t450: f64, t1578: f64, t1141: f64, t1143: f64, t1581: f64, t220: f64, t3124: f64, t3138: f64, t4293: f64, t4303: f64, t4307: f64, t4310: f64, t468: f64) -> (f64, f64) {
    let t4314 = t2785 * t1113 * t450;
    let t4317 = t1578 * t1113;
    let t4322 = t1141 * t1143 * t4307 + t1141 * t1143 * t4310 + t1141 * t1143 * t4317 + 2.0_f64 * t1581 * t3124 * t4303 - t1581 * t3138 * t4314 + t220 * t4293 * t468;
    (t4314, t4322)
}
