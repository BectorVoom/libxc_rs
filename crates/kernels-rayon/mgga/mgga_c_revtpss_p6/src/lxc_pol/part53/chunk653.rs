//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 653/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk653(t2242: f64, t38: f64, t1925: f64, t2247: f64, t644: f64, t84: f64, t77: f64, t603: f64, t607: f64) -> (f64, f64, f64, f64, f64) {
    let t6954 = t2242 * t38;
    let t6957 = t38 * t1925;
    let t6958 = t2247 * t6957;
    let t6959 = t84 * t644;
    let t6960 = t77 * t6959;
    let t6963 = t603 * t607;
    (t6954, t6957, t6958, t6960, t6963)
}
