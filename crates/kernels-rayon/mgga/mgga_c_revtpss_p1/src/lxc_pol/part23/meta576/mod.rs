//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta576 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2184;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2185;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta576(t225: f64, t23185: f64, t23187: f64, t23192: f64, t23224: f64, t10626: f64, t23114: f64, t4416: f64, t5962: f64, t23148: f64, t832: f64, t1553: f64, t1555: f64, t227: f64, t229: f64, t4415: f64, t6006: f64, t6010: f64, t6013: f64, t231: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t23227, t23235, t23238, t23241, t23244) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2184(t225, t23185, t23187, t23192, t23224, t10626, t23114, t4416, t5962, t23148, t832, t1553, t1555, t227, t229, t4415, t6006, t6010, t6013);
        let t23245 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2185(t231, t23244);
    (t23227, t23235, t23238, t23241, t23244, t23245)
}
