//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2464/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2464(t3075: f64, t3154: f64, t11671: f64, t11865: f64, t11697: f64, t11710: f64, t3091: f64, t11725: f64, t828: f64, t11706: f64, t11779: f64, t3215: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t43116 = t3154 * t3075;
    let t43121 = t11865 * t11671;
    let t43129 = t3091 * t11710 * t11697;
    let t43131 = t828 * t11725;
    let t43133 = t3091 * t43131 * t11706;
    let t43146 = t11779 * t3215;
    (t43116, t43121, t43129, t43131, t43133, t43146)
}
