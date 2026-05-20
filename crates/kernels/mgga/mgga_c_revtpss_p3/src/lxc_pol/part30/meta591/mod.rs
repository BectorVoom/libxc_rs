//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta591 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2052;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta591<F: Float>(t10073: F, t25920: F, t25938: F, t25898: F, t94889: F, t25901: F, t10115: F, t2024: F, t112: F, t843: F, t239: F, t655: F) -> (F, F, F, F, F, F) {
        let (t94919, t94921, t94922, t94931, t94974, t94975) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2052::<F>(t10073, t25920, t25938, t25898, t94889, t25901, t10115, t2024, t112, t843, t239, t655);
    (t94919, t94921, t94922, t94931, t94974, t94975)
}
