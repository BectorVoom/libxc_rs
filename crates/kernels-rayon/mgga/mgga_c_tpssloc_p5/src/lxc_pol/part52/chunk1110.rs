//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1110/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1110(t27473: f64, t7362: f64, t1215: f64, t8054: f64, t1246: f64, t1244: f64, t24760: f64, t24773: f64, t27406: f64, t27451: f64, t27455: f64, t27462: f64, t27466: f64, t27471: f64, t5064: f64, t7283: f64, t7365: f64, t7387: f64) -> f64 {
    let t27474 = t7362 * t27473;
    let t27477 = t8054 * t1215;
    let t27478 = t27477 * t1246;
    let t27480 = -0.27415567780803773942e-2_f64 * t24760 - 0.91385225936012579807e-3_f64 * t27451 - 0.82246703342411321825e-2_f64 * t7283 * t27455 + 0.73108180748810063843e-2_f64 * t27406 * t7365 - 0.27415567780803773942e-2_f64 * t7283 * t27462 - 0.27415567780803773942e-2_f64 * t7283 * t27466 + t5064 * t7387 + t1244 * t27471 - t24773 - 0.27415567780803773942e-2_f64 * t7283 * t27474 + t1244 * t27478;
    t27480
}
