//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta219 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk871;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta219(t1043: f64, t3154: f64, t4893: f64, t3117: f64, t3317: f64, t4891: f64, t357: f64, t1651: f64, t1045: f64, t999: f64, t4781: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4894, t4895, t4896, t4899, t4900, t4901, t4902, t4905, t4906, t4907, t4910, t4911) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk871(t1043, t3154, t4893, t3117, t3317, t4891, t357, t1651, t1045, t999, t4781);
    (t4894, t4895, t4896, t4899, t4900, t4901, t4902, t4905, t4906, t4907, t4910, t4911)
}
