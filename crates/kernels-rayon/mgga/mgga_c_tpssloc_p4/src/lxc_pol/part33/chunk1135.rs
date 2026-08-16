//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1135/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1135(t12571: f64, t6489: f64, t33: f64, t7440: f64, t2240: f64, t1453: f64, t22470: f64, t1982: f64, t8944: f64, t12461: f64, t2018: f64, t532: f64, t7752: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t26051 = t12571 * t6489;
    let t26083 = t33 * t7440;
    let t26084 = t2240 * t26083;
    let t26127 = t22470 * t1453;
    let t26161 = t1982 * t8944;
    let t26162 = t2018 * t12461;
    let t26167 = t532 * t7752;
    (t26051, t26083, t26084, t26127, t26161, t26162, t26167)
}
