//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1775/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1775(t1982: f64, t25460: f64, t994: f64, t1972: f64, t3223: f64, t1024: f64, t7125: f64, t3215: f64, t7117: f64, t3204: f64, t3143: f64, t3148: f64, sigma0: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t25473 = t1982 * t25460;
    let t25476 = t994 * t25460;
    let t25490 = t3223 * t1972;
    let t25495 = t1024 * t7125;
    let t25498 = t7117 * t3215;
    let t25500 = t3204 * t1972;
    let t25503 = t3143 * sigma0;
    let t25504 = t25503 * t3148;
    (t25473, t25476, t25490, t25495, t25498, t25500, t25503, t25504)
}
