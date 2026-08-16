//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1286/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1286(t1497: f64, t7719: f64, t1927: f64, t5872: f64, t2247: f64, t5826: f64, t30110: f64, t531: f64, t30974: f64, t575: f64, t2121: f64, t5819: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t108978 = t7719 * t1497;
    let t108986 = t1927 * t5872;
    let t108990 = t2247 * t5826;
    let t109173 = t531 * t30110;
    let t111419 = t30974 * t575;
    let t111453 = t2247 * t5819 * t2121;
    (t108978, t108986, t108990, t109173, t111419, t111453)
}
