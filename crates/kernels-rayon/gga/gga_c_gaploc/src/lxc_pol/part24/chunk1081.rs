//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1081/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1081(t25729: f64, t6508: f64, t2754: f64, t447: f64, t2366: f64, t1305: f64, t986: f64, t197: f64, t161: f64, t4538: f64, t599: f64, t7861: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t25730 = t6508 * t25729;
    let t25734 = t2754 * t447;
    let t25735 = t2366 * t25734;
    let t25739 = t986 * t1305;
    let t25740 = t2366 * t25739;
    let t25760 = t197 * t2754;
    let t25761 = t25760 * t161;
    let t25775 = t4538 * t986;
    let t25841 = t599 * t7861;
    (t25730, t25735, t25740, t25760, t25761, t25775, t25841)
}
