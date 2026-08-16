//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 586/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk586(t2569: f64, t885: f64, t875: f64, t296: f64) -> (f64, f64, f64, f64) {
    let t2570 = t2569 * t885;
    let t2573 = t875 * t875;
    let t2574 = 1.0_f64 / t2573;
    let t2575 = t296 * t2574;
    (t2570, t2573, t2574, t2575)
}
