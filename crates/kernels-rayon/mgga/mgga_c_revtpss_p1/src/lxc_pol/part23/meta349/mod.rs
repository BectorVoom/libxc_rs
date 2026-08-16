//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta349 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1654;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1655;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta349(t14586: f64, t836: f64, t10529: f64, t2782: f64, t4469: f64, t72: f64, t686: f64, t874: f64, t1558: f64, t2811: f64, t2482: f64, t122: f64, t2723: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let t14587 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1654(t14586, t836);
        let (t14588, t14590, t14593, t14596, t14597, t14598, t14600) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1655(t10529, t14587, t2782, t4469, t72, t686, t874, t1558, t2811, t2482, t122, t2723);
    (t14587, t14588, t14590, t14593, t14596, t14597, t14598, t14600)
}
