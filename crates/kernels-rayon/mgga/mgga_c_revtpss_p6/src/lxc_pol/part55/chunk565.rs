//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 565/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk565(t247: f64, t4816: f64, t1063: f64, t1670: f64, t3172: f64, t1041: f64, t1065: f64, t1651: f64, t906: f64, t1042: f64, t1066: f64, t4583: f64) -> (f64, f64, f64, f64) {
    let t4817 = t247 * t4816;
    let t4818 = t1063 * t4817;
    let t4820 = t3172 * t1670;
    let t4821 = t1041 * t4820;
    let t4823 = t1065 * t1651;
    let t4824 = t4823 * t906;
    let t4825 = t1042 * t4824;
    let t4830 = t1066 * t4583;
    (t4818, t4821, t4825, t4830)
}
