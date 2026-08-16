//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta724 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2780;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta724(t40113: f64, t760: f64, t2523: f64, t9419: f64, t10573: f64, t2619: f64, t2598: f64, t9321: f64, t9387: f64, t2495: f64, t39875: f64, t9367: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t40115, t40121, t40127, t40129, t40131, t40132, t40135) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2780(t40113, t760, t2523, t9419, t10573, t2619, t2598, t9321, t9387, t2495, t39875, t9367);
    (t40115, t40121, t40127, t40129, t40131, t40132, t40135)
}
