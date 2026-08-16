//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta215 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk989;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk990;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta215(t225: f64, t5710: f64, t1892: f64, t213: f64, t1357: f64, t1904: f64, t689: f64, t1903: f64, t72: f64, t686: f64, t3915: f64, t1444: f64, t4076: f64, t1882: f64, t555: f64, t4086: f64, t543: f64, t2782: f64, t1883: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5711, t5715, t5718, t5719, t5721, t5722) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk989(t225, t5710, t1892, t213, t1357, t1904, t689, t1903, t72, t686);
        let (t5723, t5728, t5735, t5737, t5738, t5740) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk990(t3915, t5722, t1444, t1903, t4076, t1882, t555, t4086, t543, t2782, t1883, t72);
    (t5711, t5715, t5718, t5719, t5721, t5722, t5723, t5728, t5735, t5737, t5738, t5740)
}
