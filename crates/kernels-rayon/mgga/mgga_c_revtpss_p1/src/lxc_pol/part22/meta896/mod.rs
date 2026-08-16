//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta896 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3088;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta896(t11675: f64, t15682: f64, t11711: f64, t15618: f64, t1043: f64, t1469: f64, t3133: f64, t3162: f64, t3115: f64, t42793: f64, t4906: f64, t11722: f64, t4834: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t53559, t53567, t53585, t53586, t53612, t53626) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3088(t11675, t15682, t11711, t15618, t1043, t1469, t3133, t3162, t3115, t42793, t4906, t11722, t4834);
    (t53559, t53567, t53585, t53586, t53612, t53626)
}
