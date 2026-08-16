//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 999/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk999(t14103: f64, t2457: f64, t9674: f64, t10073: f64, t5737: f64, t1882: f64, t4114: f64, t2482: f64, t10069: f64, t136: f64, t1892: f64, t3964: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t14104 = t14103 * t2457;
    let t14105 = t9674 * t14104;
    let t14120 = t10073 * t5737;
    let t14140 = t4114 * t1882;
    let t14141 = t2482 * t14140;
    let t14149 = t10069 * t5737;
    let t14159 = t1892 * t136;
    let t14161 = t3964 * t14159 * t2457;
    (t14104, t14105, t14120, t14141, t14149, t14159, t14161)
}
