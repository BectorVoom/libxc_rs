//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1235/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1235(t42859: f64, t487: f64, t1276: f64, t2148: f64, t13038: f64, t2142: f64, t26894: f64, t26921: f64, t1210: f64, t29193: f64, t26948: f64, t8945: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t96886 = t487 * t42859;
    let t96888 = t2148 * t96886 * t1276;
    let t96889 = t13038 * t2142;
    let t96927 = t26894 * t26921;
    let t96953 = t1210 * t26921;
    let t96979 = t1210 * t29193;
    let t96986 = t26894 * t29193;
    let t97040 = t26948 * t487;
    let t97041 = t97040 * t8945;
    (t96888, t96889, t96927, t96953, t96979, t96986, t97041)
}
