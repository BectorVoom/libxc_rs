//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1234/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1234(t243: f64, t7021: f64, t2732: f64, t1941: f64, t853: f64, t10902: f64, t27221: f64, t40419: f64, t64: f64, t9731: f64, t2710: f64, t826: f64) -> (f64, f64, f64, f64, f64) {
    let t92978 = t7021 * t243;
    let t92979 = t92978 * t2732;
    let t92981 = t1941 * t853;
    let t92982 = t92981 * t10902;
    let t92984 = t27221 * t40419;
    let t92986 = t64 * t9731;
    let t92988 = t2710 * t92986 * t826;
    (t92979, t92982, t92984, t92986, t92988)
}
