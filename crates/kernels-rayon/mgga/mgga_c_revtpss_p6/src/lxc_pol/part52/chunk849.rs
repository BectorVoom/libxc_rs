//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 849/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk849(t2466: f64, t25377: f64, t25375: f64, t1955: f64, t25308: f64, t251: f64, t7063: f64) -> (f64, f64, f64, f64) {
    let t25378 = t25377 * t2466;
    let t25379 = t25375 * t25378;
    let t25383 = t1955 * t25308;
    let t25386 = t7063 * t251;
    (t25378, t25379, t25383, t25386)
}
