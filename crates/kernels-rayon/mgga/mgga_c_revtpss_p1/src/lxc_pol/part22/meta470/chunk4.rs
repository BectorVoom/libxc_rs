//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2165/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2165(t11986: f64, t1592: f64, t247: f64, t1063: f64, t1062: f64, t11940: f64) -> (f64, f64, f64) {
    let t15711 = t247 * t11986 * t1592;
    let t15712 = t1063 * t15711;
    let t15716 = t11940 * t1062;
    (t15711, t15712, t15716)
}
