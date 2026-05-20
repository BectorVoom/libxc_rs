//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta515 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1534;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1535;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta515<F: Float>(t11921: F, t23964: F, t247: F, t4837: F, t11246: F, t23833: F, t3172: F, t1063: F, t23851: F, t1011: F, t140: F, t23873: F, t11941: F, t127: F, t24032: F, t371: F, t15671: F, t20016: F, t1025: F, t24022: F, t15993: F, t23499: F, t11875: F, t11922: F, t24012: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t79564, t79575, t79580, t79638) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1534::<F>(t11921, t23964, t247, t4837, t11246, t23833, t3172, t1063, t23851, t1011, t140, t23873);
        let (t79742, t79744, t79758, t79811, t79818) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1535::<F>(t11941, t127, t24032, t371, t15671, t20016, t1025, t24022, t1011, t15993, t23499, t11875, t11922, t24012);
    (t79564, t79575, t79580, t79638, t79742, t79744, t79758, t79811, t79818)
}
