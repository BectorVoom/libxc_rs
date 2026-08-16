//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 625/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk625(t1201: f64, t1244: f64, t2121: f64, t2152: f64, t470: f64, t7283: f64, t7361: f64, t7365: f64, t7368: f64, t7373: f64, t7378: f64, t7382: f64, t7387: f64, t7389: f64) -> f64 {
    let t7391 = t7361 - 0.27415567780803773942e-2_f64 * t7283 * t7365 - 0.82246703342411321825e-2_f64 * t7283 * t7368 + 0.82246703342411321825e-2_f64 * t7373 * t7378 + 0.82246703342411321825e-2_f64 * t2121 * t7382 + t1201 * t2152 + t1244 * t7387 + t470 * t7389;
    t7391
}
