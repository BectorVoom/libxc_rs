//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1313/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1313(t2281: f64, t2331: f64, t656: f64, t30133: f64, t576: f64, t30094: f64, t580: f64, t2212: f64, t3931: f64, t1395: f64, t8217: f64, t2205: f64, t3946: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t110140 = t2281 * t2331;
    let t110143 = t2281 * t656;
    let t110274 = t576 * t30133;
    let t110276 = t30094 * t580;
    let t110280 = t3931 * t2212;
    let t110282 = t1395 * t8217;
    let t110284 = t2205 * t3946;
    (t110140, t110143, t110274, t110276, t110280, t110282, t110284)
}
