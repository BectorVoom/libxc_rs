//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta577 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2186;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2187;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta577<F: Float>(t23168: F, t827: F, t828: F, t23172: F, t124: F, t23114: F, t800: F, t23148: F, t1544: F, t5984: F, t10673: F, t10687: F, t10692: F, t10870: F, t10900: F, t14712: F, t14716: F, t14761: F, t14765: F, t18338: F, t18340: F, t2721: F, t2730: F, t799: F, t5962: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t23253, t23257, t23262, t23263, t23266, t23267, t23275, t23278) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2186::<F>(t23168, t827, t828, t23172, t124, t23114, t800, t23148, t1544, t5984, t10673, t10687, t10692, t10870, t10900, t14712, t14716, t14761, t14765, t18338, t18340, t2721, t2730, t799);
        let t23279 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2187::<F>(t1544, t5962);
    (t23253, t23257, t23262, t23263, t23266, t23267, t23275, t23278, t23279)
}
