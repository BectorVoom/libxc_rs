//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta544 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2357;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta544<F: Float>(t17633: F, t3629: F, t3626: F, t2258: F, t3628: F, t5351: F, t3367: F, t471: F, t2251: F, t372: F, t5296: F, t5297: F, t5405: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t17634, t17635, t17638, t17639, t17640, t17643, t17644, t17645, t17646, t17649, t17650) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2357::<F>(t17633, t3629, t3626, t2258, t3628, t5351, t3367, t471, t2251, t372, t5296, t5297, t5405);
    (t17634, t17635, t17638, t17639, t17640, t17643, t17644, t17645, t17646, t17649, t17650)
}
