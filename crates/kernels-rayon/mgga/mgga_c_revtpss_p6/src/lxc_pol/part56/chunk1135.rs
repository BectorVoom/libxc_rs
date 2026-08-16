//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1135/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1135(t4424: f64, t7076: f64, t14587: f64, t25416: f64, t2747: f64, t31756: f64, t31767: f64, t4343: f64, t10779: f64, t119837: f64, t1544: f64, t119968: f64) -> (f64, f64, f64, f64, f64) {
    let t126291 = t7076 * t4424;
    let t126304 = t25416 * t14587;
    let t126319 = t31767 * t2747 * t31756 * t4343;
    let t126322 = t10779 * t119837 * t1544;
    let t126323 = t119968 * t126322;
    (t126291, t126304, t126319, t126322, t126323)
}
