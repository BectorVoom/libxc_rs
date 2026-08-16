//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta355 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1221;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta355(t1668: f64, t6258: f64, t1045: f64, t3117: f64, t1651: f64, t6299: f64, t6305: f64, t3155: f64, t3162: f64, t11765: f64, t22688: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t23992, t23993, t23994, t23997, t23998, t23999, t24007, t24008, t24009, t24012, t24013, t24016) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1221(t1668, t6258, t1045, t3117, t1651, t6299, t6305, t3155, t3162, t11765, t22688);
    (t23992, t23993, t23994, t23997, t23998, t23999, t24007, t24008, t24009, t24012, t24013, t24016)
}
