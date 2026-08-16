//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta354 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1853;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1854;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta354(t11986: f64, t247: f64, t906: f64, t1063: f64, t1062: f64, t3196: f64, t3223: f64, t1052: f64, t3147: f64, t1036: f64, t3141: f64, t3229: f64, t369: f64, t361: f64, t351: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11988, t11989, t11991, t11994) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1853(t11986, t247, t906, t1063, t1062, t3196, t3223);
        let (t11997, t11998, t11999, t12003, t12004) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1854(t1052, t3147, t1036, t3141, t3229, t369, t361, t351);
    (t11988, t11989, t11991, t11994, t11997, t11998, t11999, t12003, t12004)
}
