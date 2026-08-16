//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1243/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1243(t5410: f64, t5721: f64, t5415: f64, t5420: f64, t5728: f64, t5424: f64, t1705: f64, t5427: f64, t935: f64, t1639: f64, t1656: f64, t520: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t21044 = t5721 * t5410;
    let t21046 = t5721 * t5415;
    let t21048 = t5728 * t5420;
    let t21050 = t5728 * t5424;
    let t21060 = t1705 * t5427;
    let t21061 = t21060 * t935;
    let t21074 = t1656 * t1639 * t520;
    (t21044, t21046, t21048, t21050, t21060, t21061, t21074)
}
