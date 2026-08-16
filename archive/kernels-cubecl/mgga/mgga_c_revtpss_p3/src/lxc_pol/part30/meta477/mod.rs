//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta477 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1803;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta477<F: Float>(t14365: F, t25759: F, t1113: F, t775: F, t2430: F, t33: F, t2408: F, t890: F, t2832: F, t1940: F, t1963: F, t2403: F, t25206: F, t25436: F, t25440: F, t25445: F, t25752: F, t3351: F, t4541: F, t7087: F, t7091: F, t7200: F, t7207: F) -> (F, F, F, F, F, F, F) {
        let (t25760, t25763, t25767, t25778, t25781, t25784, t25791) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1803::<F>(t14365, t25759, t1113, t775, t2430, t33, t2408, t890, t2832, t1940, t1963, t2403, t25206, t25436, t25440, t25445, t25752, t3351, t4541, t7087, t7091, t7200, t7207);
    (t25760, t25763, t25767, t25778, t25781, t25784, t25791)
}
