//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta163 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk798;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk799;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta163<F: Float>(t3497: F, t3523: F, t1161: F, t1170: F, t1180: F, t1189: F, t3378: F, t3381: F, t3388: F, t3430: F, t3438: F, t3444: F, t3447: F, t3452: F, t3454: F, t3472: F, t3477: F, t3480: F, t3489: F, t3491: F, t3496: F, t3498: F, t3516: F, t3521: F, t435: F, t300: F, t1175: F) -> (F, F, F, F) {
        let (t3524, t3527) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk798::<F>(t3497, t3523, t1161, t1170, t1180, t1189, t3378, t3381, t3388, t3430, t3438, t3444, t3447, t3452, t3454, t3472, t3477, t3480, t3489, t3491, t3496, t3498, t3516, t3521, t435);
        let (t3528, t3530, t3531) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk799::<F>(t300, t3527, t3489, t1175);
    (t3524, t3528, t3530, t3531)
}
