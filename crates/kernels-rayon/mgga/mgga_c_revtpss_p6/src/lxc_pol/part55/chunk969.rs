//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 969/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk969(t2089: f64, t4292: f64, t670: f64, t8065: f64, t1518: f64, t7474: f64, t1519: f64, t2322: f64, t26399: f64, t28658: f64, t4254: f64, t4257: f64, t651: f64, t7235: f64, t7359: f64, t7374: f64, t7537: f64, t7539: f64, t7732: f64, t7898: f64, t7978: f64, t7988: f64, t8111: f64) -> (f64, f64, f64, f64) {
    let t28734 = t2089 * t4292;
    let t28737 = t8065 * t670;
    let t28750 = t7474 * t1518;
    let t28759 = -2.0_f64 * t1519 * t26399 - 2.0_f64 * t1519 * t28658 - 2.0_f64 * t2322 * t7978 - 2.0_f64 * t2322 * t7988 - 2.0_f64 * t28734 * t651 - 2.0_f64 * t28737 * t651 - 2.0_f64 * t28750 * t651 - 2.0_f64 * t4254 * t7978 - 2.0_f64 * t4254 * t7988 - 2.0_f64 * t4257 * t7359 - t7235 * t8111 - 2.0_f64 * t7374 * t7732 + t7537 * t7898 - t7539 * t7898;
    (t28734, t28737, t28750, t28759)
}
