//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3173/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3173(t12227: f64, t1732: f64, t12248: f64, t3433: f64, t16831: f64, t300: f64, t12429: f64, t1744: f64, t12472: f64, t5142: f64, t17150: f64, t3523: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t57795 = t12227 * t1732;
    let t57818 = t12248 * t1732;
    let t57854 = t3433 * t1732;
    let t57861 = t300 * t16831;
    let t57944 = t12429 * t1744;
    let t57972 = t5142 * t12472;
    let t58000 = t17150 * t3523;
    (t57795, t57818, t57854, t57861, t57944, t57972, t58000)
}
