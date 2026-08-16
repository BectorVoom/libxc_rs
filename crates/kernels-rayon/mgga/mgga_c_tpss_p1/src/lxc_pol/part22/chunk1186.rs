//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1186/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1186(t17946: f64, t764: f64, t1693: f64, t238: f64, t2149: f64, t2153: f64, t5547: f64, t2157: f64, t64: f64, t234: f64, t339: f64, t2165: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t17947 = t17946 * t764;
    let t17948 = 7.0_f64 / 72.0_f64 * t17947;
    let t17949 = t1693 * t238;
    let t17950 = t17949 * t2149;
    let t17952 = t5547 * t2153;
    let t17954 = t2157 * t64;
    let t17956 = t339 * t17954 * t234;
    let t17957 = t17956 * t2165;
    (t17947, t17948, t17950, t17952, t17954, t17957)
}
