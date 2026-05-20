//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta242 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk925;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk926;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta242<F: Float>(t300: F, t6541: F, t6514: F, t1765: F, t5192: F, t1188: F, t3495: F, t6518: F, t1196: F, t1179: F, t6534: F, t3520: F, t3523: F, t3546: F, t5044: F, t6423: F, t6427: F, t6431: F, t459: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t6542, t6544, t6546, t6548, t6550, t6552, t6554, t6555) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk925::<F>(t300, t6541, t6514, t1765, t5192, t1188, t3495, t6518, t1196, t1179, t6534, t3520);
        let (t6556, t6558, t6563, t6564) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk926::<F>(t3523, t6555, t1196, t3546, t5044, t6423, t6427, t6431, t459);
    (t6542, t6544, t6546, t6548, t6550, t6552, t6554, t6555, t6556, t6558, t6563, t6564)
}
