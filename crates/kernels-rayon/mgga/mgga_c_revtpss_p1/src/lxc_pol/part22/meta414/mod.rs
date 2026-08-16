//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta414 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2019;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta414(t177: f64, t4392: f64, t762: f64, t10605: f64, t162: f64, t4403: f64, t2626: f64, t4398: f64, t10439: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t14322, t14324, t14325, t14327, t14328, t14329, t14330) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2019(t177, t4392, t762, t10605, t162, t4403, t2626, t4398, t10439);
    (t14322, t14324, t14325, t14327, t14328, t14329, t14330)
}
