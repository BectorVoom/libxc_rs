//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1078/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1078(t3546: f64, t3563: f64, t2045: f64, t4580: f64, t2048: f64, t13110: f64, t539: f64, t1871: f64, t40: f64, t4579: f64, t13004: f64, t740: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t37422 = t3546 * t3563;
    let t37438 = t2045 * t4580;
    let t37441 = t2048 * t4580;
    let t37467 = t539 * t13110;
    let t37470 = t40 * t4579 * t1871;
    let t37498 = t13004 * t740;
    (t37422, t37438, t37441, t37467, t37470, t37498)
}
