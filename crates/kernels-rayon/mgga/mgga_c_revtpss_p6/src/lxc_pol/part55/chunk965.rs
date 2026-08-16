//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 965/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk965(t1312: f64, t13426: f64, t1518: f64, t18227: f64, t2055: f64, t2322: f64, t26399: f64, t27123: f64, t28219: f64, t28652: f64, t28653: f64, t28658: f64, t28683: f64, t4248: f64, t4292: f64, t5523: f64, t670: f64, t7359: f64, t7373: f64, t7889: f64, t7983: f64) -> f64 {
    let t28686 = 2.0_f64 * t1312 * t28683 + 2.0_f64 * t13426 * t2055 + 2.0_f64 * t1518 * t26399 + 2.0_f64 * t1518 * t28658 + 2.0_f64 * t18227 * t2055 + 2.0_f64 * t2055 * t27123 + 2.0_f64 * t2055 * t28219 + 2.0_f64 * t2322 * t7983 + 2.0_f64 * t28653 * t670 + 2.0_f64 * t4248 * t7373 + 2.0_f64 * t4292 * t7359 + 2.0_f64 * t5523 * t7983 + 2.0_f64 * t7373 * t7889 + t28652;
    t28686
}
