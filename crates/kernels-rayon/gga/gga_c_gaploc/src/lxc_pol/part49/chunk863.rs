//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 863/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk863(t12445: f64, t1407: f64, t2293: f64, t587: f64, t9438: f64, t9439: f64, t12449: f64, t7014: f64, t2487: f64, t9448: f64, t31182: f64, t901: f64) -> (f64, f64, f64, f64, f64) {
    let t40009 = t1407 * t12445;
    let t40013 = t587 * t9438 * t9439 * t2293;
    let t40015 = t7014 * t12449;
    let t40019 = t2487 * t9438 * t9448 * t2293;
    let t40021 = t31182 * t901;
    (t40009, t40013, t40015, t40019, t40021)
}
