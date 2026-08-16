//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1080/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1080(t4694: f64, t7030: f64, t4703: f64, t7073: f64, t4699: f64, t4637: f64, t658: f64, t13061: f64, t1998: f64, t2042: f64, t4580: f64, t1994: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t38148 = t7030 * t4694;
    let t38172 = t7073 * t4703;
    let t38174 = t7073 * t4699;
    let t38298 = t4637 * t658;
    let t38318 = t13061 * t1998;
    let t38332 = t2042 * t4580;
    let t38339 = t13061 * t1994;
    (t38148, t38172, t38174, t38298, t38318, t38332, t38339)
}
