//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta547 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1939;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1940;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta547(t29731: f64, t7160: f64, t1668: f64, t7817: f64, t1089: f64, t7821: f64, t1646: f64, t7810: f64, t7145: f64, t1976: f64, t6350: f64, t25464: f64, t7828: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t29732, t29739, t29740, t29743, t29744, t29747, t29748, t29751) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1939(t29731, t7160, t1668, t7817, t1089, t7821, t1646, t7810, t7145, t1976, t6350);
        let (t29752, t29759) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1940(t25464, t29751, t1668, t7828);
    (t29732, t29739, t29740, t29743, t29744, t29747, t29748, t29751, t29752, t29759)
}
