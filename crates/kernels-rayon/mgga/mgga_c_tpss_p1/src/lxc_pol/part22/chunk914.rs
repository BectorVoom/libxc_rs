//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 914/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk914(t672: f64, t930: f64, t925: f64, t2748: f64, t2753: f64, t361: f64, t650: f64, t242: f64, t949: f64, t946: f64, t2469: f64, t2751: f64) -> (f64, f64, f64, f64, f64) {
    let t8455 = t672 * t930;
    let t8456 = t925 * t8455;
    let t8462 = t2748 * t2753;
    let t8469 = t650 * t361;
    let t8471 = t242 * t8469 * t949;
    let t8472 = t946 * t8471;
    let t8480 = t242 * t2751 * t2469;
    (t8456, t8462, t8469, t8472, t8480)
}
