//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta369 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1344;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1345;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta369<F: Float>(t10428: F, t2414: F, t10587: F, t2496: F, t10467: F, t705: F, t707: F, t190: F, t39457: F, t706: F, t39875: F, t39894: F, t9371: F, t760: F, t39960: F, t39963: F, t2523: F, t9372: F, t10600: F, t14325: F, t2258: F, t4401: F, t606: F, t749: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t40155, t40157, t40160, t40163, t40165) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1344::<F>(t10428, t2414, t10587, t2496, t10467, t705, t707, t190, t39457, t706, t39875, t39894, t9371);
        let (t40167, t40169, t40171, t40173, t40175, t40178) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1345::<F>(t40165, t760, t39875, t39960, t39963, t2523, t9372, t10600, t14325, t2258, t4401, t606, t749);
    (t40155, t40157, t40160, t40163, t40165, t40167, t40169, t40171, t40173, t40175, t40178)
}
