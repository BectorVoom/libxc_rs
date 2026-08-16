//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta570 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1894;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta570(t26169: f64, t7702: f64, t28640: f64, t6954: f64, t1923: f64, t28089: f64, t7348: f64, t26205: f64, t26204: f64, t7719: f64, t101214: f64, t2047: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t101901, t101903, t101906, t101907, t101929, t101935) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1894(t26169, t7702, t28640, t6954, t1923, t28089, t7348, t26205, t26204, t7719, t101214, t2047);
    (t101901, t101903, t101906, t101907, t101929, t101935)
}
