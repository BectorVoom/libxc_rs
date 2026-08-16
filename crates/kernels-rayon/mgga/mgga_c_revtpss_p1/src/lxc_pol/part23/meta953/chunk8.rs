//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3171/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3171(t1038: f64, t1241: f64, t1244: f64, t24679: f64, t1252: f64, t17351: f64, t17649: f64, t17693: f64, t17799: f64, t1797: f64, t21028: f64, t21102: f64, t5287: f64, t57118: f64, t69958: f64, t70082: f64, t70088: f64, t70369: f64, t70373: f64, t70376: f64, t83033: f64, t83034: f64) -> f64 {
    let t83296 = t1241 * t1244 * t24679 * t1038;
    let t83307 = -0.85748036236139473944e-3_f64 * t17693 * t17799 * t83034 + 0.42874018118069736972e-3_f64 * t17351 * t17649 * t83033 * t21028 + 0.21722835846488666732e-1_f64 * t70082 * t1797 + 0.21722835846488666732e-1_f64 * t21102 * t5287 - 0.53100265402527852012e-1_f64 * t83296 * t1252 + 0.64311027177104605458e-3_f64 * t69958 * t1797 - 0.68598428988911579154e-2_f64 * t70088 * t1797 + 0.95275595817932748825e-4_f64 * t57118 - 0.11433071498151929859e-2_f64 * t70369 + 0.85748036236139473944e-3_f64 * t70373 - 0.17149607247227894788e-2_f64 * t70376;
    t83307
}
