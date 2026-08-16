//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta663 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2394;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta663(t41245: f64, t41306: f64, t2966: f64, t302: f64, t2969: f64, t11506: f64, t960: f64, t315: f64, t41224: f64, t11408: f64, t941: f64, t11465: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t41672, t41690, t41740, t41742, t41756, t41759, t41779, t41788) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2394(t41245, t41306, t2966, t302, t2969, t11506, t960, t315, t41224, t11408, t941, t11465);
    (t41672, t41690, t41740, t41742, t41756, t41759, t41779, t41788)
}
