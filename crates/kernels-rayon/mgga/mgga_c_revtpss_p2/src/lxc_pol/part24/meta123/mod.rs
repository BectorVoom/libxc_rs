//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta123 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk661;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta123(t2782: f64, t4496: f64, t1559: f64, t72: f64, t686: f64, t2798: f64, t225: f64, t2718: f64) -> (f64, f64, f64, f64, f64) {
        let (t4497, t4499, t4500, t4501, t4503) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk661(t2782, t4496, t1559, t72, t686, t2798, t225, t2718);
    (t4497, t4499, t4500, t4501, t4503)
}
