//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1161/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1161(t13272: f64, t1470: f64, t1497: f64, t7719: f64, t1927: f64, t5872: f64, t2247: f64, t5826: f64, t108138: f64, t96187: f64, t96236: f64, t30256: f64, t689: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t108966 = t13272 * t1470;
    let t108978 = t7719 * t1497;
    let t108986 = t1927 * t5872;
    let t108990 = t2247 * t5826;
    let t109391 = t96187 * t108138;
    let t109393 = t96236 * t108138;
    let t109396 = t30256 * t689;
    (t108966, t108978, t108986, t108990, t109391, t109393, t109396)
}
