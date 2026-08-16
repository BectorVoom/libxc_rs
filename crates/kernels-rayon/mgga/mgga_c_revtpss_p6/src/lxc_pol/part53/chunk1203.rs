//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1203/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1203(t127381: f64, t25082: f64, t8717: f64, t27833: f64, t8600: f64, t13426: f64, t8457: f64, t18227: f64, t32311: f64, t4248: f64, t28030: f64, t7003: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t127384 = 6.0_f64 * t25082 * t8717 * t127381;
    let t127385 = t27833 * t8600;
    let t127393 = t13426 * t8457;
    let t127395 = t18227 * t8457;
    let t127397 = t4248 * t32311;
    let t127399 = t28030 * t7003;
    (t127384, t127385, t127393, t127395, t127397, t127399)
}
