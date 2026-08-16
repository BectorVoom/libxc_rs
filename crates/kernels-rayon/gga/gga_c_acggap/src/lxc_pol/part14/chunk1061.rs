//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1061/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1061(t1181: f64, t5567: f64, t7351: f64, t7426: f64, t30934: f64, t9608: f64, t2001: f64, t5529: f64, t25941: f64, t599: f64, t7337: f64, t1815: f64, t372: f64) -> (f64, f64, f64, f64, f64) {
    let t38755 = t7426 * t1181 * t7351 * t5567;
    let t38757 = t30934 * t9608;
    let t38760 = t2001 * t5529;
    let t38764 = t7337 * t1181 * t599 * t25941;
    let t38766 = t1815 * t372;
    (t38755, t38757, t38760, t38764, t38766)
}
