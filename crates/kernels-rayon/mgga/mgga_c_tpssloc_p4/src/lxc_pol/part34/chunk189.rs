//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 189/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk189(t583: f64, t9: f64, t2: f64, t16: f64) -> (f64, f64, f64, f64) {
    let t584 = 1.0_f64 / t583;
    let t586 = 0.174e1_f64 * t9 * t584;
    let t587 = t9 * t2;
    let t588 = t587 * t16;
    (t584, t586, t587, t588)
}
