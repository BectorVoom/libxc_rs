//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta337 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1139;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1140;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta337(t15127: f64, t4625: f64, t698: f64, t4622: f64, t1593: f64, t2435: f64, t4584: f64, t689: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t15128, t15168, t15169, t15170, t15189) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1139(t15127, t4625, t698, t4622, t1593, t2435);
        let t15191 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1140(t4584, t689);
    (t15128, t15168, t15169, t15170, t15189, t15191)
}
