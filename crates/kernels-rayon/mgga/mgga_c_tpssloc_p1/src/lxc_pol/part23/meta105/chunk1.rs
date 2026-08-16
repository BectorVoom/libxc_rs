//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 578/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk578(t1995: f64, t241: f64, t67: f64, t1376: f64, t566: f64, t68: f64, t3787: f64, t562: f64, t193: f64, t532: f64) -> (f64, f64, f64, f64) {
    let t3869 = t241 * t1995;
    let t3870 = t3869 * t67;
    let t3886 = 1.0_f64 / t1376 / t566;
    let t3887 = t68 * t3886;
    let t3897 = t3787 * t562;
    let t3918 = t193 * t532;
    (t3870, t3887, t3897, t3918)
}
