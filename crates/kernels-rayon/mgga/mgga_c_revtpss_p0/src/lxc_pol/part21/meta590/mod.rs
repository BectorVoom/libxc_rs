//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta590 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2307;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta590(t21013: f64, t3782: f64, t12712: f64, t471: f64, t1774: f64, t3367: f64, t17934: f64, t5330: f64, t1248: f64, t3604: f64, t3670: f64, t5390: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t21017, t21028, t21035, t21049, t21119, t21203) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2307(t21013, t3782, t12712, t471, t1774, t3367, t17934, t5330, t1248, t3604, t3670, t5390);
    (t21017, t21028, t21035, t21049, t21119, t21203)
}
