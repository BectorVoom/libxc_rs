//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta351 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1692;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta351(t247: f64, t2862: f64, t3109: f64, t1063: f64, t126: f64, t3181: f64, t2853: f64, t1007: f64, t3083: f64, t1003: f64, t3080: f64, t221: f64, t346: f64, t68: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11722, t11723, t11725, t11727, t11728, t11730, t11732, t11735) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1692(t247, t2862, t3109, t1063, t126, t3181, t2853, t1007, t3083, t1003, t3080, t221, t346, t68);
    (t11722, t11723, t11725, t11727, t11728, t11730, t11732, t11735)
}
