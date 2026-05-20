//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta221 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk972;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta221<F: Float>(t12051: F, t357: F, t11239: F, t3143: F, t342: F, t3154: F, t4980: F, t994: F, t4995: F, t3057: F, t3286: F, t11627: F) -> (F, F, F, F, F, F, F, F) {
        let (t12052, t12077, t12078, t12079, t12122, t12127, t12149, t12166) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk972::<F>(t12051, t357, t11239, t3143, t342, t3154, t4980, t994, t4995, t3057, t3286, t11627);
    (t12052, t12077, t12078, t12079, t12122, t12127, t12149, t12166)
}
