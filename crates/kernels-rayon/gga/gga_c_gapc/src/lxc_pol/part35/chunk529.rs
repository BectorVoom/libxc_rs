//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 529/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk529(t3073: f64, t3076: f64, t644: f64, t998: f64, t169: f64, t442: f64, t599: f64) -> (f64, f64, f64, f64) {
    let t3077 = t3073 * t3076;
    let t3079 = t998 * t644;
    let t3080 = t169 * t3079;
    let t3081 = t442 * t599;
    (t3077, t3079, t3080, t3081)
}
