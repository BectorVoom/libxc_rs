//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta299 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1549;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1550;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1551;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta299<F: Float>(t11671: F, t3114: F, t11200: F, t225: F, t366: F, t1053: F, t3204: F, t1021: F, t3201: F, t1054: F, t2434: F, t371: F, t373: F, t367: F, t1065: F, t675: F, t247: F, t906: F, t1063: F, t1062: F, t3223: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t11933, t11940) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1549::<F>(t11671, t3114, t11200, t225);
        let (t11941, t11947, t11956, t11967, t11970, t11972, t11986) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1550::<F>(t11940, t366, t1053, t3204, t1021, t3201, t1054, t2434, t371, t373, t367, t1065, t675);
        let (t11988, t11989, t11994) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1551::<F>(t11986, t247, t906, t1063, t1062, t3223);
    (t11933, t11940, t11941, t11947, t11956, t11967, t11970, t11972, t11986, t11988, t11989, t11994)
}
