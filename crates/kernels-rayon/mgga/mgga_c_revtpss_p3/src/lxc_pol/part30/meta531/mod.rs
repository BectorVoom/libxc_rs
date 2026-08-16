//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta531 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1946;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta531(t1916: f64, t7331: f64, t7334: f64, t1459: f64, t7950: f64, t1936: f64, t670: f64, t1518: f64, t572: f64, t26123: f64, t4292: f64, t7330: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t28259, t28261, t28263, t28264, t28265, t28267, t28268, t28270, t28271) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1946(t1916, t7331, t7334, t1459, t7950, t1936, t670, t1518, t572, t26123, t4292, t7330);
    (t28259, t28261, t28263, t28264, t28265, t28267, t28268, t28270, t28271)
}
