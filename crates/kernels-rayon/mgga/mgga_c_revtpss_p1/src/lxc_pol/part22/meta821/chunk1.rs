//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2936/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2936(t10069: f64, t14124: f64, t14129: f64, t14231: f64, t10014: f64, t14216: f64, t13921: f64, t4101: f64, t686: f64, t72: f64, t10139: f64, t136: f64, t2457: f64, t5659: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t47978 = t10069 * t14124;
    let t47980 = t10069 * t14129;
    let t47985 = t10069 * t14231;
    let t47995 = t10014 * t14216;
    let t47999 = t4101 * t13921 * t72 * t686;
    let t48003 = t10139 * t5659 * t136 * t2457;
    (t47978, t47980, t47985, t47995, t47999, t48003)
}
