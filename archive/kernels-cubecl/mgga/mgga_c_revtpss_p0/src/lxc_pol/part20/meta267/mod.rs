//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta267 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1115;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1116;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta267<F: Float>(t3154: F, t905: F, t606: F, t11659: F, t3092: F, t3095: F, t1052: F, t360: F, t3089: F, t1087: F, t3090: F, t3278: F, t3133: F, t73: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t11660, t11661, t11662, t11663, t11666, t11667, t11670, t11671) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1115::<F>(t3154, t905, t606, t11659, t3092, t3095, t1052, t360, t3089);
        let (t11672, t11675, t11678) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1116::<F>(t1087, t11671, t3090, t3278, t3133, t73);
    (t11660, t11661, t11662, t11663, t11666, t11667, t11670, t11671, t11672, t11675, t11678)
}
