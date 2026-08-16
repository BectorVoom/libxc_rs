//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1076 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3856;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3857;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1076(t47093: f64, t39989: f64, t47084: f64, t47086: f64, t47088: f64, t47092: f64, t47096: f64, t74114: f64, t74115: f64, t74116: f64, t74117: f64, t74119: f64, t74120: f64, t74121: f64, t74122: f64, t74123: f64, t74124: f64, t74125: f64, t47099: f64, t22212: f64, t2626: f64, t1320: f64, t22195: f64, t47101: f64, t48313: f64, t47110: f64, t47113: f64, t47119: f64, t47125: f64, t40067: f64, t40072: f64, t47098: f64, t47109: f64, t47116: f64, t47118: f64, t47122: f64, t47124: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t74126, t74127) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3856(t47093, t39989, t47084, t47086, t47088, t47092, t47096, t74114, t74115, t74116, t74117, t74119, t74120, t74121, t74122, t74123, t74124, t74125);
        let (t74129, t74131, t74133, t74134, t74135, t74136, t74137, t74138, t74139, t74140) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3857(t47099, t22212, t2626, t1320, t22195, t47101, t48313, t47110, t47113, t47119, t47125, t40067, t40072, t47098, t47109, t47116, t47118, t47122, t47124);
    (t74126, t74127, t74129, t74131, t74133, t74134, t74135, t74136, t74137, t74138, t74139, t74140)
}
