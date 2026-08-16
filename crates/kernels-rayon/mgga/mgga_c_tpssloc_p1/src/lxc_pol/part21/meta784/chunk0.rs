//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2718/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2718(t54460: f64, t54462: f64, t39851: f64, t39857: f64, t54467: f64, t54469: f64, t54471: f64, t40221: f64, t40225: f64, t19573: f64, t588: f64, t592: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t57218 = 160.0_f64 * t54460;
    let t57219 = 240.0_f64 * t54462;
    let t57220 = 24.0_f64 * t39851;
    let t57221 = 64.0_f64 * t39857;
    let t57222 = 0.20508037716432813315e4_f64 * t54467;
    let t57223 = 0.23392894490538584828e1_f64 * t54469;
    let t57224 = 0.69263436422725855034e2_f64 * t54471;
    let t57225 = 8.0_f64 * t40221;
    let t57226 = 24.0_f64 * t40225;
    let t57227 = t588 * t19573;
    let t57228 = 8.0_f64 * t57227;
    let t57229 = t592 * t19573;
    (t57218, t57219, t57220, t57221, t57222, t57223, t57224, t57225, t57226, t57228, t57229)
}
