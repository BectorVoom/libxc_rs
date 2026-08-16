//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta404 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1998;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta404(t2470: f64, t5721: f64, t3915: f64, t1445: f64, t5599: f64, t689: f64, t2435: f64, t5600: f64, t1426: f64, t1893: f64, t786: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t14090, t14091, t14094, t14096, t14097, t14099, t14100) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1998(t2470, t5721, t3915, t1445, t5599, t689, t2435, t5600, t1426, t1893, t786);
    (t14090, t14091, t14094, t14096, t14097, t14099, t14100)
}
