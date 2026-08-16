//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta273 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1487;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta273(t10565: f64, t158: f64, t755: f64, t9586: f64, t2619: f64, t2622: f64, t2629: f64, t9863: f64, t123: f64, t752: f64) -> (f64, f64, f64, f64, f64) {
        let (t10566, t10568, t10569, t10577, t10578) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1487(t10565, t158, t755, t9586, t2619, t2622, t2629, t9863, t123, t752);
    (t10566, t10568, t10569, t10577, t10578)
}
