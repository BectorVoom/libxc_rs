//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta515 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1534;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1535;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta515(t11921: f64, t23964: f64, t247: f64, t4837: f64, t11246: f64, t23833: f64, t3172: f64, t1063: f64, t23851: f64, t1011: f64, t140: f64, t23873: f64, t11941: f64, t127: f64, t24032: f64, t371: f64, t15671: f64, t20016: f64, t1025: f64, t24022: f64, t15993: f64, t23499: f64, t11875: f64, t11922: f64, t24012: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t79564, t79575, t79580, t79638) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1534(t11921, t23964, t247, t4837, t11246, t23833, t3172, t1063, t23851, t1011, t140, t23873);
        let (t79742, t79744, t79758, t79811, t79818) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1535(t11941, t127, t24032, t371, t15671, t20016, t1025, t24022, t1011, t15993, t23499, t11875, t11922, t24012);
    (t79564, t79575, t79580, t79638, t79742, t79744, t79758, t79811, t79818)
}
