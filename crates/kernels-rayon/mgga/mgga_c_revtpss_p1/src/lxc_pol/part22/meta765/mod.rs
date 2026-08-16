//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta765 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2847;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta765(t342: f64, t43471: f64, t3043: f64, t3298: f64, t16551: f64, t994: f64, t16558: f64, t16505: f64, t11627: f64, t42859: f64, t16553: f64, t3133: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t43472, t43512, t43520, t43524, t43528, t43536, t43537, t43568) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2847(t342, t43471, t3043, t3298, t16551, t994, t16558, t16505, t11627, t42859, t16553, t3133);
    (t43472, t43512, t43520, t43524, t43528, t43536, t43537, t43568)
}
