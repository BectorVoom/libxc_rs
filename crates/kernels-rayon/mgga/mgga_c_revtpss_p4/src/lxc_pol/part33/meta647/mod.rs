//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta647 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2096;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta647(t17544: f64, t7618: f64, t17373: f64, t29040: f64, t17769: f64, t7624: f64, t104695: f64, t13142: f64, t17384: f64, t26867: f64, t26827: f64, t5362: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t104756, t104768, t104770, t104774, t104793, t104815) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2096(t17544, t7618, t17373, t29040, t17769, t7624, t104695, t13142, t17384, t26867, t26827, t5362);
    (t104756, t104768, t104770, t104774, t104793, t104815)
}
