//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2406/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2406(t271: f64, t2852: f64, t1054: f64, t11970: f64, t11986: f64, t828: f64, t11631: f64, t905: f64, t606: f64, t1086: f64, t11223: f64, t3090: f64) -> (f64, f64, f64, f64, f64) {
    let t43222 = 1.0_f64 / t271 / t2852;
    let t43238 = t1054 * t11970;
    let t43240 = t828 * t11986;
    let t43253 = t11631 * t905;
    let t43254 = t43253 * t606;
    let t43285 = t11223 * t1086 * t3090;
    (t43222, t43238, t43240, t43254, t43285)
}
