//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1393/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1393(t2434: f64, t371: f64, t482: f64, t481: f64, t3172: f64, t3605: f64, t3600: f64, t11262: f64, t1251: f64, t1247: f64, t3704: f64, t3708: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12898 = t371 * t2434 * t482;
    let t12900 = 0.63517063878621832551e-4_f64 * t481 * t12898;
    let t12901 = t3172 * t3605;
    let t12902 = t3600 * t12901;
    let t12904 = t11262 * t1251;
    let t12905 = t1247 * t12904;
    let t12907 = t3708 * t3704;
    (t12898, t12900, t12901, t12902, t12904, t12905, t12907)
}
