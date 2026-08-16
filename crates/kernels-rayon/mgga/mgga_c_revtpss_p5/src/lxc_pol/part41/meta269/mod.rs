//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta269 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1017;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta269(t3994: f64, t808: f64, t9845: f64, t521: f64, t9342: f64, t14: f64, t588: f64, t2496: f64, t4038: f64, t123: f64, t1330: f64, t2630: f64) -> (f64, f64, f64, f64, f64) {
        let (t9847, t9854, t9856, t9858, t9861) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1017(t3994, t808, t9845, t521, t9342, t14, t588, t2496, t4038, t123, t1330, t2630);
    (t9847, t9854, t9856, t9858, t9861)
}
