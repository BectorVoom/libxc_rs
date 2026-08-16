//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta203 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk937;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta203(t760: f64, t9318: f64, t162: f64, t9544: f64, t158: f64, t755: f64, t9586: f64, t2629: f64, t9863: f64, t9866: f64, t9575: f64, t9572: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10554, t10565, t10566, t10568, t10577, t10582, t10584, t10586) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk937(t760, t9318, t162, t9544, t158, t755, t9586, t2629, t9863, t9866, t9575, t9572);
    (t10554, t10565, t10566, t10568, t10577, t10582, t10584, t10586)
}
