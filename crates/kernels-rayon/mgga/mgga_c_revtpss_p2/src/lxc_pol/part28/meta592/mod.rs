//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta592 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2063;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta592(t1399: f64, t2434: f64, t25880: f64, t25899: f64, t3924: f64, t676: f64, t2022: f64, t9646: f64, t9648: f64, t25875: f64, t94394: f64, t94398: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t94634, t94635, t94640, t94641, t94648, t94649, t94650) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2063(t1399, t2434, t25880, t25899, t3924, t676, t2022, t9646, t9648, t25875, t94394, t94398);
    (t94634, t94635, t94640, t94641, t94648, t94649, t94650)
}
