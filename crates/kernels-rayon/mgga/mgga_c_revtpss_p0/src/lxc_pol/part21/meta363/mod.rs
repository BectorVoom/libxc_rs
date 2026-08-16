//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta363 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1725;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1726;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1727;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1728;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta363(t3124: f64, t3173: f64, t11231: f64, t4806: f64, t1042: f64, t1065: f64, t675: f64, t247: f64, t906: f64, t1063: f64, t1062: f64, t3196: f64, t3223: f64, t1052: f64, t3147: f64, t1036: f64, t3141: f64, t3229: f64, t369: f64, t361: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11980, t11982, t11983, t11986) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1725(t3124, t3173, t11231, t4806, t1042, t1065, t675);
        let (t11988, t11989, t11991) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1726(t11986, t247, t906, t1063, t1062, t3196);
        let t11994 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1727(t1062, t3223);
        let (t11997, t11998, t11999, t12003) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1728(t1052, t3147, t1036, t3141, t3229, t369, t361);
    (t11980, t11982, t11983, t11986, t11988, t11989, t11991, t11994, t11997, t11998, t11999, t12003)
}
