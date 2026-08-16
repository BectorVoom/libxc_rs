//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta332 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1161;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1162;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta332(t150: f64, t23210: f64, t190: f64, t1469: f64, t18305: f64, t4401: f64, t14613: f64, t6002: f64, t22671: f64, t706: f64, t10592: f64, t10596: f64, t10604: f64, t10611: f64, t23193: f64, t23213: f64, t9542: f64, t225: f64, t23185: f64, t23187: f64, t23192: f64, t10626: f64, t23114: f64, t4416: f64, t5962: f64, t23148: f64, t832: f64, t1553: f64, t1555: f64, t227: f64, t229: f64, t4415: f64, t6006: f64, t6010: f64, t6013: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t23214, t23215, t23216, t23218, t23220, t23221, t23223, t23224) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1161(t150, t23210, t190, t1469, t18305, t4401, t14613, t6002, t22671, t706, t10592, t10596, t10604, t10611, t23193, t23213, t9542);
        let (t23227, t23235, t23238, t23241, t23244) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1162(t225, t23185, t23187, t23192, t23224, t10626, t23114, t4416, t5962, t23148, t832, t1553, t1555, t227, t229, t4415, t6006, t6010, t6013);
    (t23214, t23215, t23216, t23218, t23220, t23221, t23223, t23227, t23235, t23238, t23241, t23244)
}
