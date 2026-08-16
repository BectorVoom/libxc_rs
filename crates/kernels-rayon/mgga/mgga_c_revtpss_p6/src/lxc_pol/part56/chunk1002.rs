//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1002/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1002(t34007: f64, t572: f64, t1916: f64, t8614: f64, t1518: f64, t32374: f64, t1568: f64, t3140: f64, t8477: f64, t1892: f64, t1501: f64, t1936: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t34009 = 12.0_f64 * t572 * t34007;
    let t34010 = t1916 * t8614;
    let t34011 = 3.0_f64 * t34010;
    let t34012 = t32374 * t1518;
    let t34013 = t572 * t34012;
    let t34014 = 6.0_f64 * t34013;
    let t34074 = t1568 * t3140;
    let t34075 = t8477 * t34074;
    let t34230 = t1892 * t3140;
    let t34231 = t8477 * t34230;
    let t34258 = t1501 * t1936;
    (t34009, t34011, t34012, t34014, t34074, t34075, t34230, t34231, t34258)
}
