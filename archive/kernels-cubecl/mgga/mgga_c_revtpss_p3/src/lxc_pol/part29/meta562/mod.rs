//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta562 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1905;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1906;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta562<F: Float>(t13826: F, t7271: F, t13923: F, t7264: F, t14036: F, t25997: F, t13946: F, t26028: F, t13941: F, t94423: F, t14005: F, t13834: F, t27940: F, t13841: F, t5706: F, t94429: F, t1941: F, t9817: F, t48662: F, t5651: F, t7028: F, t9736: F, t13985: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t98176, t98178, t98180, t98182, t98185, t98187, t98189) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1905::<F>(t13826, t7271, t13923, t7264, t14036, t25997, t13946, t26028, t13941, t94423, t14005, t13834, t27940);
        let (t98191, t98193, t98197, t98200, t98202) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1906::<F>(t13841, t26028, t5706, t94429, t1941, t9817, t48662, t5651, t7028, t9736, t13985, t94423);
    (t98176, t98178, t98180, t98182, t98185, t98187, t98189, t98191, t98193, t98197, t98200, t98202)
}
