//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1990/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1990(t670: f64, t7356: f64, t2051: f64, t2371: f64, t102019: f64, t13426: f64, t13514: f64, t1518: f64, t18227: f64, t2055: f64, t26153: f64, t26399: f64, t28653: f64, t28658: f64, t4248: f64, t4292: f64, t49686: f64, t7359: f64, t7373: f64, t75485: f64, t75667: f64, t95357: f64) -> (f64, f64, f64) {
    let t102714 = t7356 * t670;
    let t102719 = t2051 * t2371;
    let t102738 = 4.0_f64 * t102019 * t670 + 4.0_f64 * t102714 * t1518 + 2.0_f64 * t102719 * t1518 + 4.0_f64 * t13426 * t7373 + 2.0_f64 * t13514 * t7359 + 2.0_f64 * t1518 * t95357 + 4.0_f64 * t18227 * t7373 + 2.0_f64 * t2055 * t49686 + 2.0_f64 * t2055 * t75485 + 4.0_f64 * t2055 * t75667 + 2.0_f64 * t2371 * t28653 + 2.0_f64 * t26153 * t4248 + 4.0_f64 * t26399 * t4292 + 4.0_f64 * t28658 * t4292;
    (t102714, t102719, t102738)
}
