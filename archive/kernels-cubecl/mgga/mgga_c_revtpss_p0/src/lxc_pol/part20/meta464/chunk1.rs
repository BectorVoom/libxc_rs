//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1765/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1765<F: Float>(t1389: F, t3964: F, t40604: F, t3961: F, t9741: F, t13783: F, t1388: F, t1390: F, t1399: F, t3934: F, t46532: F, t46682: F, t47282: F, t47284: F, t47286: F, t47296: F, t47298: F, t47302: F, t47304: F, t47306: F, t47318: F, t47320: F, t47325: F, t47329: F, t47333: F, t5673: F, t828: F, t9984: F) -> F {
    let t47337 = F::cast_from(0.11344944493805280483e-2_f64) * t3964 * t40604 * t1389;
    let t47338 = t9741 * t3961;
    let t47340 = -F::cast_from(0.15246000842785598467e-3_f64) * t47282 - F::cast_from(0.48018900292238105408e-1_f64) * t47284 + F::cast_from(0.12004725073059526352e-1_f64) * t47286 - F::cast_from(0.85748036236139473944e-3_f64) * t3934 * t5673 * t46682 * t1399 - F::cast_from(0.30492001685571196935e-3_f64) * t47296 + F::cast_from(0.81312004494856525159e-3_f64) * t47298 + F::cast_from(0.30492001685571196936e-2_f64) * t47302 - F::cast_from(0.34013387707001991332e-1_f64) * t47304 + F::cast_from(0.40015750243531754508e-2_f64) * t47306 - F::cast_from(0.64311027177104605458e-3_f64) * t1388 * t1390 * t828 * t46532 - F::cast_from(0.51448821741683684368e-1_f64) * t3934 * t13783 * t9984 * t1399 - F::cast_from(0.17149607247227894789e-3_f64) * t47318 + F::cast_from(0.36585828794086175548e-2_f64) * t47320 + F::cast_from(0.17149607247227894789e-2_f64) * t47325 - F::cast_from(0.34299214494455789577e-3_f64) * t47329 - F::cast_from(0.50820002809285328224e-4_f64) * t47333 + t47337 - F::cast_from(35.0_f64) / F::cast_from(36.0_f64) * t47338;
    t47340
}
