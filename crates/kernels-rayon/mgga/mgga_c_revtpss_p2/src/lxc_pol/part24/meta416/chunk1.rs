//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1362/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1362(t225: f64, t42059: f64, t366: f64, t2857: f64, t3154: f64, t271: f64, t2852: f64, t41296: f64, t11986: f64, t828: f64, t11631: f64, t905: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t43154 = t42059 * t225;
    let t43155 = t43154 * t366;
    let t43174 = t3154 * t2857;
    let t43222 = 1.0_f64 / t271 / t2852;
    let t43223 = t43222 * t41296;
    let t43240 = t828 * t11986;
    let t43253 = t11631 * t905;
    (t43154, t43155, t43174, t43223, t43240, t43253)
}
