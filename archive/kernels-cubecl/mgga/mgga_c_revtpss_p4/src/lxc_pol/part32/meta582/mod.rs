//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta582 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1910;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta582<F: Float>(t98146: F, t98152: F, t98156: F, t98168: F, t98180: F, t98185: F, t98187: F, t98193: F, t98202: F, t98206: F, t98222: F, t98226: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t102488, t102490, t102492, t102499, t102505, t102508, t102509, t102512, t102516, t102518, t102528, t102530) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1910::<F>(t98146, t98152, t98156, t98168, t98180, t98185, t98187, t98193, t98202, t98206, t98222, t98226);
    (t102488, t102490, t102492, t102499, t102505, t102508, t102509, t102512, t102516, t102518, t102528, t102530)
}
