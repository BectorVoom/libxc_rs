//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2182/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2182(t1912: f64, t46452: f64, t82143: f64, t82145: f64, t82150: f64, t855: f64, t858: f64, t87029: f64, t87033: f64, t87039: f64, t87042: f64, t87047: f64, t87050: f64, t87094: f64, t87146: f64, t87524: f64, t87562: f64, t87606: f64, t87656: f64, t87694: f64, t87735: f64) -> f64 {
    let t87741 = t87029 + 0.19190897446562641759e-1_f64 * t82143 - 0.16449340668482264365e-1_f64 * t87033 - t46452 * t1912 - 0.6579736267392905746e-1_f64 * t87039 + 0.38381794893125283518e-1_f64 * t82145 - t87042 + 0.38381794893125283518e-1_f64 * t82150 + 0.82246703342411321825e-2_f64 * t87047 - 0.2302907693587517011e0_f64 * t87050 - t855 * t858 * (t87094 + t87146 + t87524 + t87562 + t87606 + t87656 + t87694 + t87735);
    t87741
}
