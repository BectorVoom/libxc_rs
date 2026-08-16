//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta635 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2203;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta635(t13435: f64, t7741: f64, t2322: f64, t28042: f64, t13440: f64, t5523: f64, t25191: f64, t7898: f64, t1937: f64, t49686: f64, t75667: f64, t13426: f64, t6993: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t101534, t101536, t101538, t101540, t101546, t101548, t101550, t101552) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2203(t13435, t7741, t2322, t28042, t13440, t5523, t25191, t7898, t1937, t49686, t75667, t13426, t6993);
    (t101534, t101536, t101538, t101540, t101546, t101548, t101550, t101552)
}
