//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1248/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1248(t17930: f64, t21262: f64, t1288: f64, t1364: f64, t30: f64, t4701: f64, t17949: f64, t4708: f64, t4712: f64, t5547: f64, t17956: f64, t4718: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t21263 = t17930 * t21262;
    let t21266 = t1288 * t1364;
    let t21270 = t30 * t4701;
    let t21274 = t17949 * t4708;
    let t21276 = t5547 * t4712;
    let t21278 = t17956 * t4718;
    (t21263, t21266, t21270, t21274, t21276, t21278)
}
