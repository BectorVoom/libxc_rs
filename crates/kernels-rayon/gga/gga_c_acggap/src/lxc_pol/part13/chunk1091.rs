//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1091/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1091(t1165: f64, t20775: f64, t30698: f64, t604: f64, t1181: f64, t22710: f64, t7413: f64, t599: f64, t4680: f64, t7337: f64, t8774: f64, t4263: f64, t7351: f64, t7575: f64) -> (f64, f64, f64, f64, f64) {
    let t34969 = t30698 * t1165 * t604 * t20775;
    let t34973 = t7413 * t1181 * t604 * t22710;
    let t34977 = t30698 * t1181 * t599 * t20775;
    let t34980 = t7337 * t4680 * t8774;
    let t34984 = t7575 * t1181 * t7351 * t4263;
    (t34969, t34973, t34977, t34980, t34984)
}
