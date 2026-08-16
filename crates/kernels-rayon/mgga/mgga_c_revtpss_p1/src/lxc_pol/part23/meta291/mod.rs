//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta291 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1523;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1524;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta291(t11132: f64, t240: f64, t624: f64, t281: f64, t283: f64, t3252: f64, t276: f64, t285: f64, t273: f64, t2439: f64, t931: f64, t2922: f64, t913: f64, t275: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11304, t11334, t11335, t11337, t11338, t11341, t11354, t11358, t11366, t11384) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1523(t11132, t240, t624, t281, t283, t3252, t276, t285, t273, t2439, t931, t2922, t913);
        let t11385 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1524(t11384, t275);
    (t11304, t11334, t11335, t11337, t11338, t11341, t11354, t11358, t11366, t11384, t11385)
}
