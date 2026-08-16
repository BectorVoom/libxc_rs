//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta202 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk936;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta202(t2441: f64, t9303: f64, t10115: f64, t258: f64, t2453: f64, t2464: f64, t251: f64, t4503: f64, t786: f64, t2797: f64, t760: f64, t9323: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t10501, t10503, t10504, t10529, t10530, t10535, t10552) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk936(t2441, t9303, t10115, t258, t2453, t2464, t251, t4503, t786, t2797, t760, t9323);
    (t10501, t10503, t10504, t10529, t10530, t10535, t10552)
}
