//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1618/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1618(t1675: f64, t3331: f64, t1695: f64, t3377: f64, t11297: f64, t11350: f64, t11361: f64, t11365: f64, t14958: f64, t15048: f64, t15165: f64, t15168: f64, t15172: f64, t15179: f64, t15182: f64, t15185: f64, t15204: f64, t3334: f64, t3357: f64, t3376: f64, t3401: f64, t436: f64, t4840: f64, t4862: f64) -> f64 {
    let t15207 = t1675 * t3331;
    let t15210 = t1695 * t3377;
    let t15213 = 0.64327917994770140268e2_f64 * t3357 * t15165 + 0.32163958997385070134e2_f64 * t3357 * t15168 + 0.2069040516770936012e4_f64 * t11350 * t15172 - 0.23392894490538584828e1_f64 * t11297 * t4840 + 0.34631718211362927518e2_f64 * t11361 * t4862 - 0.23392894490538584828e1_f64 * t3376 * t15179 - 0.11696447245269292414e1_f64 * t3376 * t15182 - 0.10389515463408878255e3_f64 * t11365 * t15185 - 0.310907e-1_f64 * t15204 * t436 + t14958 - 2.0_f64 * t15207 * t3334 + 0.35089341735807877242e1_f64 * t3401 * t15210 + t15048;
    t15213
}
