//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1765/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1765(t1389: f64, t3964: f64, t40604: f64, t3961: f64, t9741: f64, t13783: f64, t1388: f64, t1390: f64, t1399: f64, t3934: f64, t46532: f64, t46682: f64, t47282: f64, t47284: f64, t47286: f64, t47296: f64, t47298: f64, t47302: f64, t47304: f64, t47306: f64, t47318: f64, t47320: f64, t47325: f64, t47329: f64, t47333: f64, t5673: f64, t828: f64, t9984: f64) -> f64 {
    let t47337 = 0.11344944493805280483e-2_f64 * t3964 * t40604 * t1389;
    let t47338 = t9741 * t3961;
    let t47340 = -0.15246000842785598467e-3_f64 * t47282 - 0.48018900292238105408e-1_f64 * t47284 + 0.12004725073059526352e-1_f64 * t47286 - 0.85748036236139473944e-3_f64 * t3934 * t5673 * t46682 * t1399 - 0.30492001685571196935e-3_f64 * t47296 + 0.81312004494856525159e-3_f64 * t47298 + 0.30492001685571196936e-2_f64 * t47302 - 0.34013387707001991332e-1_f64 * t47304 + 0.40015750243531754508e-2_f64 * t47306 - 0.64311027177104605458e-3_f64 * t1388 * t1390 * t828 * t46532 - 0.51448821741683684368e-1_f64 * t3934 * t13783 * t9984 * t1399 - 0.17149607247227894789e-3_f64 * t47318 + 0.36585828794086175548e-2_f64 * t47320 + 0.17149607247227894789e-2_f64 * t47325 - 0.34299214494455789577e-3_f64 * t47329 - 0.50820002809285328224e-4_f64 * t47333 + t47337 - 35.0_f64 / 36.0_f64 * t47338;
    t47340
}
