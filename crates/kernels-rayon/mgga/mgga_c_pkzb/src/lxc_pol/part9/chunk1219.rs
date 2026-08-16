//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1219/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1219(t20917: f64, t20957: f64, t21002: f64, t21040: f64, t21121: f64, t21171: f64, t21218: f64, t21258: f64, t237: f64, t5893: f64, t730: f64, t7535: f64) -> (f64, f64) {
    let t21262 = t237 * (t20917 + t20957 + t21002 + t21040 + t21121 + t21171 + t21218 + t21258);
    let t21265 = 0.51947577317044391277e2_f64 * t730 * t7535 * t5893;
    (t21262, t21265)
}
