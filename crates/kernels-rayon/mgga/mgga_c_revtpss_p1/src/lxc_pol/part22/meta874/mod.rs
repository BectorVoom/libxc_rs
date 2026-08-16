//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta874 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3037;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3038;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta874(t10069: f64, t14482: f64, t15003: f64, t41020: f64, t14939: f64, t213: f64, t4470: f64, t786: f64, t867: f64, t2467: f64, t14567: f64, t2453: f64, t10538: f64, t14662: f64, t251: f64, t225: f64, t40321: f64, t822: f64, t686: f64, t72: f64, t874: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t51264, t51268, t51272, t51276, t51277, t51297) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3037(t10069, t14482, t15003, t41020, t14939, t213, t4470, t786, t867, t2467, t14567, t2453);
        let (t51298, t51306, t51320, t51332, t51339) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3038(t10538, t51297, t14662, t251, t213, t225, t40321, t14939, t822, t686, t72, t874);
    (t51264, t51268, t51272, t51276, t51277, t51297, t51298, t51306, t51320, t51332, t51339)
}
