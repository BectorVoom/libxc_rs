//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2460/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2460(t11671: f64, t3278: f64, t12020: f64, t3168: f64, t2434: f64, t246: f64, t1041: f64, t1046: f64, t11256: f64, t11258: f64, t3172: f64, t11727: f64, t3188: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t42967 = t3278 * t11671;
    let t42970 = t12020 * t3168;
    let t42994 = t246 * t2434;
    let t42996 = t1041 * t42994 * t1046;
    let t43003 = t11256 * t3172 * t11258;
    let t43017 = t3188 * t11727;
    (t42967, t42970, t42994, t42996, t43003, t43017)
}
