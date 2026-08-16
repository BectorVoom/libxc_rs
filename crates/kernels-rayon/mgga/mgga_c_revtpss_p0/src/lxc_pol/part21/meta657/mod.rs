//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta657 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2447;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2448;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta657(t3105: f64, t3223: f64, t11960: f64, t351: f64, t361: f64, t369: f64, t1041: f64, t11262: f64, t3135: f64, t1033: f64, t1036: f64, t1038: f64, t1063: f64, t11160: f64, t247: f64, t3109: f64, t11620: f64, t73: f64, t12166: f64, t15905: f64, t994: f64, t11662: f64, t11710: f64, t4892: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t42571, t42576, t42580, t42584) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2447(t3105, t3223, t11960, t351, t361, t369, t1041, t11262, t3135, t1033, t1036, t1038);
        let (t42606, t42610, t42621, t42637) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2448(t1063, t11160, t247, t3109, t11620, t73, t12166, t15905, t994, t11662, t11710, t4892);
    (t42571, t42576, t42580, t42584, t42606, t42610, t42621, t42637)
}
