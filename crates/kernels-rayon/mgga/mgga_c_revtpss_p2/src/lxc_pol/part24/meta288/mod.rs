//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta288 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1069;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta288(t19976: f64, t3115: f64, t4817: f64, t4834: f64, t127: f64, t371: f64, t6337: f64, t3205: f64, t6276: f64, t1025: f64, t4845: f64, t4858: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t19977, t20005, t20016, t20017, t20020, t20021, t20025) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1069(t19976, t3115, t4817, t4834, t127, t371, t6337, t3205, t6276, t1025, t4845, t4858);
    (t19977, t20005, t20016, t20017, t20020, t20021, t20025)
}
