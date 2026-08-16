//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 359/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk359(t94: f64, t102: f64, t177: f64, t738: f64, t745: f64, t746: f64, t761: f64, t118: f64, t187: f64, t677: f64, t763: f64, t200: f64, t262: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2341 = 1.0_f64 / t94;
    let t2349 = 1.0_f64 / t102;
    let t2367 = t738 * t177;
    let t2368 = 1.0_f64 / t2367;
    let t2369 = t745 * t745;
    let t2371 = t2368 * t2369 * t746;
    let t2373 = 0.11696447245269292414e1_f64 * t761 * t2371;
    let t2374 = t187 * t118;
    let t2375 = t677 * t763;
    let t2377 = 0.10843581300301739842e-1_f64 * t2374 * t2375;
    let t2378 = t200 * t262;
    (t2341, t2349, t2368, t2369, t2371, t2373, t2375, t2377, t2378)
}
