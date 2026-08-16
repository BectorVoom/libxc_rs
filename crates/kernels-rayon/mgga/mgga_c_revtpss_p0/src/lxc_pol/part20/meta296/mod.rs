//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta296 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1170;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta296(t1145: f64, t12273: f64, t141: f64, t10326: f64, t1121: f64) -> (f64, f64, f64) {
        let (t12274, t12275, t12277) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1170(t1145, t12273, t141, t10326, t1121);
    (t12274, t12275, t12277)
}
