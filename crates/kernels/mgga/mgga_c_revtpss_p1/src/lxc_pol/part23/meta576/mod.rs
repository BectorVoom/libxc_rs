//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta576 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2184;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2185;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta576<F: Float>(t225: F, t23185: F, t23187: F, t23192: F, t23224: F, t10626: F, t23114: F, t4416: F, t5962: F, t23148: F, t832: F, t1553: F, t1555: F, t227: F, t229: F, t4415: F, t6006: F, t6010: F, t6013: F, t231: F) -> (F, F, F, F, F, F) {
        let (t23227, t23235, t23238, t23241, t23244) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2184::<F>(t225, t23185, t23187, t23192, t23224, t10626, t23114, t4416, t5962, t23148, t832, t1553, t1555, t227, t229, t4415, t6006, t6010, t6013);
        let t23245 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2185::<F>(t231, t23244);
    (t23227, t23235, t23238, t23241, t23244, t23245)
}
