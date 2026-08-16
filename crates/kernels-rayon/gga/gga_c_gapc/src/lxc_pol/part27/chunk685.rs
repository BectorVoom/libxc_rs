//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 685/1310 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk685(t481: f64, t818: f64, t4: f64, t5: f64, t2188: f64, t2546: f64, t186: f64, t932: f64) -> (f64, f64, f64, f64) {
    let t7208 = t481 * t818;
    let t7216 = t4 * t5;
    let t7241 = t2546 * t2188;
    let t7259 = t932 * t186;
    (t7208, t7216, t7241, t7259)
}
