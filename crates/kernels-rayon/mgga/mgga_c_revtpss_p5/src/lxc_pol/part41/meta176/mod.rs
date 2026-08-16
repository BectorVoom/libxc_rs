//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta176 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk738;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta176(t366: f64, t4797: f64, t1065: f64, t2857: f64, t4181: f64, t1042: f64, t2852: f64, t3181: f64, t1592: f64, t3109: f64, t247: f64, t1063: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4798, t4801, t4802, t4803, t4806, t4807, t4808, t4817, t4818) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk738(t366, t4797, t1065, t2857, t4181, t1042, t2852, t3181, t1592, t3109, t247, t1063);
    (t4798, t4801, t4802, t4803, t4806, t4807, t4808, t4817, t4818)
}
