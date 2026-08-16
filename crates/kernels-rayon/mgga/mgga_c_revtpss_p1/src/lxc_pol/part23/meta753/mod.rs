//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta753 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2542;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta753(t52035: f64, t52037: f64, t11223: f64, t1678: f64, t1041: f64, t11262: f64, t4868: f64, t3201: f64, t4794: f64, t4798: f64, t343: f64, t44: f64, t816: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t53252, t53253, t53281, t53294, t53300, t53318, t53320) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2542(t52035, t52037, t11223, t1678, t1041, t11262, t4868, t3201, t4794, t4798, t343, t44, t816);
    (t53252, t53253, t53281, t53294, t53300, t53318, t53320)
}
