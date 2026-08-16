//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta289 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1156;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta289<F: Float>(t12123: F, t3318: F, t1043: F, t3153: F, t3133: F, t4982: F, t1071: F, t1089: F, t999: F, t3046: F, t3286: F, t3057: F) -> (F, F, F, F, F, F, F, F) {
        let (t12128, t12131, t12132, t12133, t12137, t12143, t12146, t12149) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1156::<F>(t12123, t3318, t1043, t3153, t3133, t4982, t1071, t1089, t999, t3046, t3286, t3057);
    (t12128, t12131, t12132, t12133, t12137, t12143, t12146, t12149)
}
