//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta566 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1890;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta566(t13272: f64, t607: f64, t10301: f64, t1470: f64, t2247: f64, t4181: f64, t4187: f64, t94976: f64, t1513: f64, t94975: f64, t28036: f64, t94978: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t101230, t101237, t101240, t101243, t101448, t101451, t101453) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1890(t13272, t607, t10301, t1470, t2247, t4181, t4187, t94976, t1513, t94975, t28036, t94978);
    (t101230, t101237, t101240, t101243, t101448, t101451, t101453)
}
