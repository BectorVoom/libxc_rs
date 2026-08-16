//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta498 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1499;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1500;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta498(t4321: f64, t6072: f64, t689: f64, t23383: f64, t2465: f64, t686: f64, t72: f64, t10995: f64, t23403: f64, t212: f64, t23359: f64, t780: f64, t23177: f64, t2798: f64, t14568: f64, t18730: f64, t14586: f64, t6016: f64, t10529: f64, t2782: f64, t233: f64, t869: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t76051, t76058, t76062, t76081) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1499(t4321, t6072, t689, t23383, t2465, t686, t72, t10995, t23403, t212, t23359, t780);
        let (t76100, t76104, t76108, t76117) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1500(t23177, t2798, t686, t72, t14568, t18730, t14586, t6016, t10529, t2782, t233, t23359, t689, t869);
    (t76051, t76058, t76062, t76081, t76100, t76104, t76108, t76117)
}
