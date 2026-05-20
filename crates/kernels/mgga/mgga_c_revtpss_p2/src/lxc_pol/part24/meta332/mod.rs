//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta332 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1161;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1162;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta332<F: Float>(t150: F, t23210: F, t190: F, t1469: F, t18305: F, t4401: F, t14613: F, t6002: F, t22671: F, t706: F, t10592: F, t10596: F, t10604: F, t10611: F, t23193: F, t23213: F, t9542: F, t225: F, t23185: F, t23187: F, t23192: F, t10626: F, t23114: F, t4416: F, t5962: F, t23148: F, t832: F, t1553: F, t1555: F, t227: F, t229: F, t4415: F, t6006: F, t6010: F, t6013: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t23214, t23215, t23216, t23218, t23220, t23221, t23223, t23224) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1161::<F>(t150, t23210, t190, t1469, t18305, t4401, t14613, t6002, t22671, t706, t10592, t10596, t10604, t10611, t23193, t23213, t9542);
        let (t23227, t23235, t23238, t23241, t23244) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1162::<F>(t225, t23185, t23187, t23192, t23224, t10626, t23114, t4416, t5962, t23148, t832, t1553, t1555, t227, t229, t4415, t6006, t6010, t6013);
    (t23214, t23215, t23216, t23218, t23220, t23221, t23223, t23227, t23235, t23238, t23241, t23244)
}
