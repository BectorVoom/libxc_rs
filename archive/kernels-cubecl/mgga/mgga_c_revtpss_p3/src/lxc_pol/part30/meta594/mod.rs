//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta594 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2055;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta594<F: Float>(t2122: F, t92569: F, t25163: F, t7575: F, t92576: F, t92584: F, t45958: F, t7565: F, t10301: F, t26754: F, t2247: F, t26781: F, t38: F) -> (F, F, F, F, F, F, F) {
        let (t96752, t96757, t96760, t96765, t96773, t96776, t96792) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2055::<F>(t2122, t92569, t25163, t7575, t92576, t92584, t45958, t7565, t10301, t26754, t2247, t26781, t38);
    (t96752, t96757, t96760, t96765, t96773, t96776, t96792)
}
