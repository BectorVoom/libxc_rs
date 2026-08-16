//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta341 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1643;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta341(t1532: f64, t2609: f64, t2398: f64, t4305: f64, t177: f64, t4392: f64, t762: f64, t10605: f64, t162: f64, t2626: f64, t4398: f64, t10439: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t14312, t14317, t14322, t14324, t14325, t14328, t14330) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1643(t1532, t2609, t2398, t4305, t177, t4392, t762, t10605, t162, t2626, t4398, t10439);
    (t14312, t14317, t14322, t14324, t14325, t14328, t14330)
}
