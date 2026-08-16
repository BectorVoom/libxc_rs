//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1068/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1068(t1469: f64, t32591: f64, t8442: f64, t1493: f64, t32600: f64, t8621: f64, t1892: f64, t3140: f64, t8477: f64, t1501: f64, t1936: f64, t4248: f64, t8749: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34176 = t32591 * t1469;
    let t34177 = t8442 * t34176;
    let t34181 = t8621 * t32600 * t1493;
    let t34230 = t1892 * t3140;
    let t34231 = t8477 * t34230;
    let t34258 = t1501 * t1936;
    let t34377 = t4248 * t8749;
    (t34177, t34181, t34230, t34231, t34258, t34377)
}
