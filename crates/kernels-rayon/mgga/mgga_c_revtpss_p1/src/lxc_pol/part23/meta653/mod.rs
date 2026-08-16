//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta653 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2381;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta653(t2677: f64, t40710: f64, t234: f64, t9801: f64, t10887: f64, t136: f64, t2475: f64, t220: f64, t2482: f64, t2668: f64, t823: f64, t159: f64, t33127: f64, t64: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t40711, t40721, t40722, t40724, t40725, t40731, t40735) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2381(t2677, t40710, t234, t9801, t10887, t136, t2475, t220, t2482, t2668, t823, t159, t33127, t64);
    (t40711, t40721, t40722, t40724, t40725, t40731, t40735)
}
