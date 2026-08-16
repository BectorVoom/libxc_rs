//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta584 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1936;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta584(t27799: f64, t61155: f64, t1711: f64, t2832: f64, t1113: f64, t4537: f64, t13392: f64, t603: f64, t13396: f64, t13405: f64, t4237: f64, t644: f64, t77: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t101086, t101093, t101099, t101129, t101132, t101139, t101156) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1936(t27799, t61155, t1711, t2832, t1113, t4537, t13392, t603, t13396, t13405, t4237, t644, t77);
    (t101086, t101093, t101099, t101129, t101132, t101139, t101156)
}
