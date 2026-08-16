//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1356/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1356(t283: f64, t2852: f64, t66: f64, t11821: f64, t41270: f64, t11144: f64, t3252: f64, t11852: f64, t126: f64, t12166: f64, t15905: f64, t994: f64) -> (f64, f64, f64, f64, f64) {
    let t42471 = 1.0_f64 / t283 / t2852;
    let t42472 = t66 * t42471;
    let t42508 = t11821 * t41270;
    let t42518 = t3252 * t11144;
    let t42534 = t126 * t11852;
    let t42621 = t994 * t12166 * t15905;
    (t42472, t42508, t42518, t42534, t42621)
}
