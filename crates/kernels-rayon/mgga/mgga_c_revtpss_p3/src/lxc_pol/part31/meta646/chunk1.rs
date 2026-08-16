//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2113/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2113(t18482: f64, t25270: f64, t18478: f64, t27261: f64, t18531: f64, t25245: f64, t18432: f64, t93025: f64, t18440: f64, t25227: f64, t2661: f64, t103287: f64, t106030: f64, t106033: f64, t106035: f64, t106037: f64, t106040: f64, t106042: f64, t99012: f64) -> f64 {
    let t106044 = t25270 * t18482;
    let t106046 = t27261 * t18478;
    let t106048 = t25245 * t18531;
    let t106050 = t93025 * t18432;
    let t106053 = t2661 * t25227 * t18440;
    let t106055 = t99012 - 0.28582678745379824648e-4_f64 * t106030 + 0.14291339372689912324e-4_f64 * t106033 + t103287 - 0.17149607247227894789e-2_f64 * t106035 - 0.10164000561857065645e-3_f64 * t106037 + 0.14291339372689912324e-4_f64 * t106040 + 0.20007875121765877254e-2_f64 * t106042 - 0.17149607247227894789e-1_f64 * t106044 - 0.68598428988911579156e-2_f64 * t106046 - 0.25410001404642664113e-4_f64 * t106048 + 0.50820002809285328225e-4_f64 * t106050 - 0.11433071498151929859e-3_f64 * t106053;
    t106055
}
