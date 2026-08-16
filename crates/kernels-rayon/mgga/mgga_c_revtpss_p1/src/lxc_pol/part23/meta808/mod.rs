//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta808 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2642;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2643;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta808(t18784: f64, t2465: f64, t686: f64, t72: f64, t4481: f64, t51276: f64, t6042: f64, t786: f64, t867: f64, t2467: f64, t14480: f64, t252: f64, t2782: f64, t4533: f64, t14991: f64, t50208: f64, t14485: f64, t14987: f64, t18657: f64, t213: f64, t14983: f64, t18392: f64, t262: f64, t18838: f64, t2411: f64, t18969: f64, t698: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t63062, t63064, t63084, t63085, t63091) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2642(t18784, t2465, t686, t72, t4481, t51276, t6042, t786, t867, t2467, t14480, t252, t2782, t4533);
        let (t63094, t63099, t63103, t63109, t63146, t63160, t63240) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2643(t14991, t50208, t14485, t14987, t18657, t213, t14983, t18392, t262, t18838, t2411, t18969, t698);
    (t63062, t63064, t63084, t63085, t63091, t63094, t63099, t63103, t63109, t63146, t63160, t63240)
}
