//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 910/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk910(t2178: f64, t3539: f64, t1045: f64, t9132: f64, t157: f64, t1984: f64, t2097: f64, t2179: f64, t525: f64, t9438: f64, t1985: f64, t597: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t49562 = t3539 * t2178;
    let t49622 = t9132 * t1045;
    let t50235 = t1984 * t157;
    let t50240 = t2097 * t2179;
    let t50249 = t525 * t157;
    let t50260 = t1045 * t9438;
    let t50268 = t1985 * t597;
    (t49562, t49622, t50235, t50240, t50249, t50260, t50268)
}
