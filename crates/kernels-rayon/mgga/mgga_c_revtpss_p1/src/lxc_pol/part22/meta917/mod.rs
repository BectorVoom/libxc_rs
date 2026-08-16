//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta917 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3126;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta917(t342: f64, t378: f64, t43536: f64, t11631: f64, t43350: f64, t16558: f64, t989: f64, t1071: f64, t12166: f64, t12077: f64, t43346: f64, t42872: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t55569, t55570, t55575, t55579, t55583, t55593, t55594) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3126(t342, t378, t43536, t11631, t43350, t16558, t989, t1071, t12166, t12077, t43346, t42872);
    (t55569, t55570, t55575, t55579, t55583, t55593, t55594)
}
