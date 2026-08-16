//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1238/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1238(t10474: f64, t82514: f64, t10482: f64, t3032: f64, t3127: f64, t3131: f64, t221: f64, t697: f64, t1926: f64) -> (f64, f64, f64, f64, f64) {
    let t82515 = t82514 * t10474;
    let t82516 = t3032 * t10482;
    let t82541 = t82514 * t3127;
    let t82542 = t3032 * t3131;
    let t82631 = t221 * t697;
    let t82632 = t1926 * t82631;
    (t82515, t82516, t82541, t82542, t82632)
}
