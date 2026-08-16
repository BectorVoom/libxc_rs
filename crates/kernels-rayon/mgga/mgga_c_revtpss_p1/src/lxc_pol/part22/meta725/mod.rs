//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta725 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2781;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta725(t40135: f64, t760: f64, t10565: f64, t606: f64, t706: f64, t717: f64, t10587: f64, t2496: f64, t39875: f64, t39894: f64, t9371: f64, t39960: f64, t39963: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t40137, t40139, t40150, t40156, t40165, t40167, t40169) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2781(t40135, t760, t10565, t606, t706, t717, t10587, t2496, t39875, t39894, t9371, t39960, t39963);
    (t40137, t40139, t40150, t40156, t40165, t40167, t40169)
}
