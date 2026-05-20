//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta575 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2182;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2183;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta575<F: Float>(t45: F, t57: F, t14441: F, t10446: F, t22671: F, t22688: F, t4377: F, t5825: F, t78: F, t10457: F, t4384: F, t81: F, t162: F, t187: F, zeta_threshold: F, t150: F, t190: F, t1469: F, t18305: F, t4401: F, t14613: F, t6002: F, t706: F, t10592: F, t10596: F, t10604: F, t10611: F, t9542: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t23193, t23210, t23211, t23213) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2182::<F>(t45, t57, t14441, t10446, t22671, t22688, t4377, t5825, t78, t10457, t4384, t81, t162, t187, zeta_threshold);
        let (t23214, t23215, t23216, t23218, t23220, t23221, t23223, t23224) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2183::<F>(t150, t23210, t190, t1469, t18305, t4401, t14613, t6002, t22671, t706, t10592, t10596, t10604, t10611, t23193, t23213, t9542);
    (t23193, t23210, t23211, t23213, t23214, t23215, t23216, t23218, t23220, t23221, t23223, t23224)
}
