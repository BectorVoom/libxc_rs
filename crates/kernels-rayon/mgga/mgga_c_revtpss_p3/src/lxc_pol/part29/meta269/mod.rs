//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta269 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1115;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1116;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta269(t1358: f64, t7492: f64, t689: f64, t2098: f64, t786: f64, t1364: f64, t7250: f64, t7257: f64, t7260: f64, t7267: f64, t7253: f64, t7265: f64, t7272: f64, t225: f64, t2097: f64, t213: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t7493, t7495, t7496, t7498, t7499, t7501, t7502, t7504, t7506) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1115(t1358, t7492, t689, t2098, t786, t1364, t7250, t7257, t7260, t7267, t7253, t7265, t7272);
        let (t7507, t7511) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1116(t225, t7506, t2097, t213);
    (t7493, t7495, t7496, t7498, t7499, t7501, t7502, t7504, t7506, t7507, t7511)
}
