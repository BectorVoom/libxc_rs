//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 870/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk870(t14103: f64, t2457: f64, t9674: f64, t10073: f64, t5737: f64, t10069: f64, t136: f64, t1892: f64, t3964: f64, t2435: f64, t5760: f64, t3999: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t14104 = t14103 * t2457;
    let t14105 = t9674 * t14104;
    let t14120 = t10073 * t5737;
    let t14149 = t10069 * t5737;
    let t14159 = t1892 * t136;
    let t14161 = t3964 * t14159 * t2457;
    let t14166 = t2435 * t5760;
    let t14171 = t3999 * t1892;
    (t14104, t14105, t14120, t14149, t14161, t14166, t14171)
}
