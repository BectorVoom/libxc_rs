//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta348 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1835;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1836;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta348(t1063: f64, t11727: f64, t1007: f64, t3083: f64, t1003: f64, t3080: f64, t221: f64, t346: f64, t68: f64, t345: f64, t247: f64, t2858: f64, t3109: f64, t140: f64, t3247: f64, t1011: f64, t3254: f64, t3237: f64, t245: f64, t3089: f64, t3088: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11728, t11730, t11732, t11735, t11737, t11744) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1835(t1063, t11727, t1007, t3083, t1003, t3080, t221, t346, t68, t345, t247, t2858, t3109);
        let (t11745, t11753, t11756, t11763, t11772, t11773) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1836(t1063, t11744, t140, t3247, t1011, t3254, t3237, t245, t3089, t3088);
    (t11728, t11730, t11732, t11735, t11737, t11744, t11745, t11753, t11756, t11763, t11772, t11773)
}
