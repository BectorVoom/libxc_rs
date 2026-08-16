//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2078/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2078(t1096: f64, t357: f64, t1976: f64, t4743: f64, t27543: f64, t342: f64, t4778: f64, t8521: f64, t1078: f64, t42859: f64, t1983: f64, t3143: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t99566 = t357 * t1096;
    let t99629 = t4743 * t1976;
    let t99666 = t342 * t27543;
    let t99675 = t4778 * t8521;
    let t99682 = t42859 * t1078;
    let t99684 = t1983 * t99682 * t3143;
    (t99566, t99629, t99666, t99675, t99682, t99684)
}
