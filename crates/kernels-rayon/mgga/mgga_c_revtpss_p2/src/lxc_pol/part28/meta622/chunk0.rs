//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2201/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2201(t4778: f64, t8521: f64, t1078: f64, t42859: f64, t1983: f64, t3143: f64, t11249: f64, t27641: f64, t1032: f64, t4930: f64, t994: f64, t15669: f64, t1976: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t99675 = t4778 * t8521;
    let t99682 = t42859 * t1078;
    let t99684 = t1983 * t99682 * t3143;
    let t99685 = t27641 * t11249;
    let t99708 = t4930 * t1032;
    let t99709 = t994 * t99708;
    let t99721 = t15669 * t1976;
    (t99675, t99682, t99684, t99685, t99708, t99709, t99721)
}
