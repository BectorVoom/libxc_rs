//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta898 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3090;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta898(t42865: f64, t72: f64, t3088: f64, t43472: f64, t43401: f64, t11710: f64, t15969: f64, t4892: f64, t1062: f64, t15655: f64, t11643: f64, t15707: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t53667, t53668, t53669, t53676, t53690, t53692, t53710) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3090(t42865, t72, t3088, t43472, t43401, t11710, t15969, t4892, t1062, t15655, t11643, t15707);
    (t53667, t53668, t53669, t53676, t53690, t53692, t53710)
}
