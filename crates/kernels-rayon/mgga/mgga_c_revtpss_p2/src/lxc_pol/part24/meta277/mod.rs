//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta277 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1051;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta277(t18725: f64, t686: f64, t2798: f64, t5978: f64, t72: f64, t14568: f64, t4500: f64, t18699: f64, t231: f64, t2783: f64, t2782: f64, t18677: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t18726, t18727, t18729, t18730, t18731, t18733, t18738, t18739, t18742) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1051(t18725, t686, t2798, t5978, t72, t14568, t4500, t18699, t231, t2783, t2782, t18677);
    (t18726, t18727, t18729, t18730, t18731, t18733, t18738, t18739, t18742)
}
