//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 767/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk767(t332: f64, t4597: f64, t1297: f64, t455: f64, t52: f64, t339: f64, t454: f64, t1128: f64, t5072: f64, t242: f64, t5068: f64, t5078: f64, t5080: f64, t5084: f64, t5116: f64, t5119: f64, t5185: f64, t5187: f64, t5189: f64, t5193: f64, t5197: f64, t5201: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5223 = t4597 * t332;
    let t5229 = 1.0_f64 / t52 / t455 / t1297;
    let t5231 = t339 * t454 * t5229;
    let t5234 = t1128 * t5072;
    let t5235 = t242 * t5234;
    let t5238 = t1128 * t5068;
    let t5239 = t242 * t5238;
    let t5242 = -t5078 + t5080 - t5084 + t5116 + t5119 + t5185 + t5187 - t5189 + t5193 - t5197 - t5201;
    (t5223, t5229, t5231, t5235, t5239, t5242)
}
