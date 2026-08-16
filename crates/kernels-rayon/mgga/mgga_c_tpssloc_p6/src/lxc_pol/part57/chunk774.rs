//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 774/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk774(t1528: f64, t17052: f64, t17092: f64, t1912: f64, t25036: f64, t25188: f64, t25348: f64, t259: f64, t26591: f64, t28265: f64, t28269: f64, t28274: f64, t28278: f64, t28282: f64, t28289: f64, t28296: f64, t28300: f64, t4147: f64, t4268: f64, t7517: f64, t7538: f64) -> f64 {
    let t28304 = -0.82246703342411321824e-2_f64 * t25036 + 4.0_f64 * t4268 * t7517 - 0.82246703342411321825e-2_f64 * t28265 + 0.3289868133696452873e-1_f64 * t28269 - t26591 + 0.82246703342411321825e-2_f64 * t28274 - 0.16449340668482264365e-1_f64 * t28278 - 2.0_f64 * t25348 * t1528 + t28282 * t259 + 4.0_f64 * t4147 * t7517 - 2.0_f64 * t17092 * t1912 - 0.3289868133696452873e-1_f64 * t28289 - 2.0_f64 * t4147 * t7538 - t17052 * t1912 + 0.16449340668482264365e-1_f64 * t28296 + 0.49348022005446793095e-1_f64 * t28300 - 2.0_f64 * t25188 * t1528;
    t28304
}
