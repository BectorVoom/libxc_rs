//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta181 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk924;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta181(t1359: f64, t9292: f64, t1363: f64, t9288: f64, t1362: f64, t3911: f64, t3920: f64, t3957: f64, t3961: f64, t124: f64, t9628: f64, t800: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t9691, t9692, t9694, t9695, t9697, t9699, t9700) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk924(t1359, t9292, t1363, t9288, t1362, t3911, t3920, t3957, t3961, t124, t9628, t800);
    (t9691, t9692, t9694, t9695, t9697, t9699, t9700)
}
