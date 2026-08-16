//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta612 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2364;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta612(t10573: f64, t2619: f64, t2598: f64, t9321: f64, t760: f64, t2523: f64, t9387: f64, t2495: f64, t39875: f64, t9367: f64, t10565: f64, t606: f64, t706: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t40127, t40129, t40131, t40132, t40135, t40137, t40139) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2364(t10573, t2619, t2598, t9321, t760, t2523, t9387, t2495, t39875, t9367, t10565, t606, t706);
    (t40127, t40129, t40131, t40132, t40135, t40137, t40139)
}
