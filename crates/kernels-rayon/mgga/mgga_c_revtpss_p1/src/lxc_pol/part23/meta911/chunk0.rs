//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2926/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2926(t141: f64, t2908: f64, t77588: f64, t77592: f64, t77525: f64, t77529: f64, t63533: f64, t63538: f64, t63541: f64, t63543: f64, t63545: f64, t63547: f64, t63549: f64, t63551: f64) -> (f64, f64, f64, f64, f64) {
    let t77829 = t141 * t2908 * t77588;
    let t77832 = t141 * t2908 * t77592;
    let t77835 = t141 * t2908 * t77525;
    let t77838 = t141 * t2908 * t77529;
    let t77846 = -0.91983333333333333334e-1_f64 * t63533 + 0.5519e0_f64 * t63538 - 0.99342e0_f64 * t77829 + 0.49671e0_f64 * t77832 - 0.82785e-1_f64 * t77835 - 0.82785e-1_f64 * t77838 - 0.33114e0_f64 * t63541 + 0.5519e-1_f64 * t63543 - 0.27595e0_f64 * t63545 - 0.33114e0_f64 * t63547 + 0.11038e0_f64 * t63549 + 0.73586666666666666666e-1_f64 * t63551;
    (t77829, t77832, t77835, t77838, t77846)
}
