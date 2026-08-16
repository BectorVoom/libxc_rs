//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1140/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1140(t1455: f64, t1921: f64, t571: f64, t5808: f64, t575: f64, t6936: f64, t5883: f64, t648: f64, t1501: f64, t670: f64, t6765: f64, t1843: f64, t4292: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t18184 = 2.0_f64 * t1455 * t1921;
    let t18186 = 2.0_f64 * t571 * t5808;
    let t18219 = t6936 * t575;
    let t18220 = t648 * t5883;
    let t18227 = t1501 * t670;
    let t18232 = t6765 * t670;
    let t18235 = t1843 * t4292;
    (t18184, t18186, t18219, t18220, t18227, t18232, t18235)
}
