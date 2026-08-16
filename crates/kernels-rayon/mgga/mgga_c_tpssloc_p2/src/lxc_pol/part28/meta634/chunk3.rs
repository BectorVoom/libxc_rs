//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 2010/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2010(t90604: f64, t90609: f64, t16030: f64, t24082: f64, t24088: f64, t24095: f64, t24147: f64, t26996: f64, t3758: f64, t5215: f64, t5321: f64, t5326: f64, t7199: f64, t80738: f64, t84400: f64, t90626: f64, t90634: f64) -> (f64, f64, f64) {
    let t93404 = 0.76763589786250567036e-1_f64 * t90604;
    let t93407 = 0.9869604401089358619e-1_f64 * t90609;
    let t93431 = -0.82246703342411321825e-2_f64 * t80738 - t84400 + 0.16449340668482264365e-1_f64 * t90626 + 4.0_f64 * t5215 * t24147 + 2.0_f64 * t5215 * t24088 + 4.0_f64 * t24082 * t5326 + 4.0_f64 * t24095 * t5326 + 4.0_f64 * t16030 * t7199 + 2.0_f64 * t5321 * t24088 - 0.9869604401089358619e-1_f64 * t90634 + 4.0_f64 * t3758 * t26996;
    (t93404, t93407, t93431)
}
