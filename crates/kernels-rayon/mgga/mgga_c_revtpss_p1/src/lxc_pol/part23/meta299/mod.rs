//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta299 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1549;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1550;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1551;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta299(t11671: f64, t3114: f64, t11200: f64, t225: f64, t366: f64, t1053: f64, t3204: f64, t1021: f64, t3201: f64, t1054: f64, t2434: f64, t371: f64, t373: f64, t367: f64, t1065: f64, t675: f64, t247: f64, t906: f64, t1063: f64, t1062: f64, t3223: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11933, t11940) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1549(t11671, t3114, t11200, t225);
        let (t11941, t11947, t11956, t11967, t11970, t11972, t11986) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1550(t11940, t366, t1053, t3204, t1021, t3201, t1054, t2434, t371, t373, t367, t1065, t675);
        let (t11988, t11989, t11994) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1551(t11986, t247, t906, t1063, t1062, t3223);
    (t11933, t11940, t11941, t11947, t11956, t11967, t11970, t11972, t11986, t11988, t11989, t11994)
}
