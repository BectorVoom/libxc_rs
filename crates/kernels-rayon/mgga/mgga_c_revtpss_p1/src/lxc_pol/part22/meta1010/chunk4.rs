//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3467/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3467(t1063: f64, t11986: f64, t247: f64, t6100: f64, t20054: f64, t3106: f64, t3075: f64, t5819: f64, t2251: f64, t5825: f64) -> (f64, f64, f64, f64) {
    let t65357 = t1063 * t247 * t11986 * t6100;
    let t65359 = t3106 * t20054;
    let t65365 = t5819 * t3075;
    let t65370 = t5825 * t2251;
    (t65357, t65359, t65365, t65370)
}
