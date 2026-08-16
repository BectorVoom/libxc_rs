//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta448 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1677;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta448(t14365: f64, t25759: f64, t1113: f64, t775: f64, t2430: f64, t33: f64, t2408: f64, t890: f64, t2832: f64, t4135: f64, t4147: f64, t112: f64, t239: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t25760, t25763, t25767, t25778, t25781, t25784, t25802, t25821) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1677(t14365, t25759, t1113, t775, t2430, t33, t2408, t890, t2832, t4135, t4147, t112, t239);
    (t25760, t25763, t25767, t25778, t25781, t25784, t25802, t25821)
}
