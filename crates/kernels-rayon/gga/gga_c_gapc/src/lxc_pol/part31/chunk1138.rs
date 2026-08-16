//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1138/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1138(t11948: f64, t30095: f64, t11798: f64, t16720: f64, t3284: f64, t11387: f64, t16676: f64, t16677: f64, t11794: f64, t7420: f64, t11320: f64, t2619: f64, t7921: f64) -> (f64, f64, f64, f64, f64) {
    let t33167 = t11948 * t30095;
    let t33170 = t11798 * t3284 * t16720;
    let t33173 = t16676 * t11387 * t16677;
    let t33175 = t11794 * t7420;
    let t33179 = t2619 * t11320 * t7921;
    (t33167, t33170, t33173, t33175, t33179)
}
