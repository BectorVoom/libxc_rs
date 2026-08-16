//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 845/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk845(t1241: f64, t8087: f64, t1238: f64, t1761: f64, t2121: f64, t2124: f64, t2155: f64, t4945: f64, t498: f64, t5055: f64, t7282: f64, t7283: f64, t7351: f64, t7999: f64, t8003: f64, t8006: f64, t8011: f64, t8015: f64, t8018: f64, t8055: f64, t8061: f64) -> (f64, f64) {
    let t8088 = t1241 * t8087;
    let t8090 = -0.21932454224643019153e-1_f64 * t7999 * t2124 + t7282 - 0.27415567780803773942e-2_f64 * t7283 * t8003 - 0.82246703342411321825e-2_f64 * t7283 * t8006 + 0.82246703342411321825e-2_f64 * t2121 * t8011 - 0.82246703342411321825e-2_f64 * t7283 * t8015 + t8018 * t498 + t8055 * t498 - t7351 * t1761 - t4945 * t2155 - t5055 * t2155 + 2.0_f64 * t1238 * t8061 - t1238 * t8088;
    (t8088, t8090)
}
