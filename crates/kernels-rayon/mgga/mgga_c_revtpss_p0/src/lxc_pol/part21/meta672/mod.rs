//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta672 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2473;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta672(t16505: f64, t994: f64, t11627: f64, t42859: f64, t342: f64, t11620: f64, t4982: f64, t12077: f64, t989: f64, t12153: f64, t3057: f64, t3043: f64, t3316: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t43528, t43536, t43537, t43562, t43574, t43598, t43611) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2473(t16505, t994, t11627, t42859, t342, t11620, t4982, t12077, t989, t12153, t3057, t3043, t3316);
    (t43528, t43536, t43537, t43562, t43574, t43598, t43611)
}
