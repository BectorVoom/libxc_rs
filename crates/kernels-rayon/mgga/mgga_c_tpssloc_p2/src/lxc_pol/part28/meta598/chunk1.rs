//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1898/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1898(t24994: f64, t6875: f64, t26351: f64, t6883: f64, t1992: f64, t26355: f64, t80650: f64, t22635: f64, t26354: f64, t3911: f64, t22751: f64, t26186: f64) -> (f64, f64, f64, f64, f64) {
    let t90442 = t6875 * t24994;
    let t90459 = t6883 * t26351;
    let t90462 = t1992 * t80650 * t26355;
    let t90466 = t1992 * t22635 * t26354 * t3911;
    let t90468 = t22751 * t26186;
    (t90442, t90459, t90462, t90466, t90468)
}
