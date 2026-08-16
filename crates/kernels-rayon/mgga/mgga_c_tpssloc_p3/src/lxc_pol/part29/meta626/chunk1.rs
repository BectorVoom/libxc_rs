//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2069/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2069(t11745: f64, t24729: f64, t2132: f64, t24746: f64, t86192: f64, t10401: f64, t24739: f64, t3610: f64, t3624: f64, t24740: f64, t3604: f64, t11838: f64, t7310: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t86299 = t24729 * t11745;
    let t86313 = t2132 * t86192 * t24746;
    let t86323 = t24739 * t10401;
    let t86324 = t3610 * t86323;
    let t86327 = t3624 * t86323;
    let t86330 = t3604 * t24740;
    let t86341 = t7310 * t11838;
    (t86299, t86313, t86324, t86327, t86330, t86341)
}
