//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1391/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1391(t12773: f64, t3625: f64, t3624: f64, t3746: f64, t3618: f64, t828: f64, t1260: f64, t3650: f64, t3588: f64, t73: f64, t1209: f64, t3781: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12774 = t3625 * t12773;
    let t12784 = t3746 * t3624;
    let t12787 = t828 * t3618;
    let t12800 = t3650 * t1260;
    let t12803 = t3588 * t73;
    let t12808 = t1209 * t3781;
    (t12774, t12784, t12787, t12800, t12803, t12808)
}
