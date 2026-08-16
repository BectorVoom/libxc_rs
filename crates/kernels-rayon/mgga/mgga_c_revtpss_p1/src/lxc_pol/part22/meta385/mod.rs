//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta385 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1951;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta385(t9552: f64, t9559: f64, t1317: f64, t5567: f64, t9564: f64, t9566: f64, t9578: f64, t9580: f64, t4147: f64, t5778: f64, t2496: f64, t5571: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13640, t13641, t13643, t13644, t13645, t13646, t13647, t13648, t13652) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1951(t9552, t9559, t1317, t5567, t9564, t9566, t9578, t9580, t4147, t5778, t2496, t5571);
    (t13640, t13641, t13643, t13644, t13645, t13646, t13647, t13648, t13652)
}
