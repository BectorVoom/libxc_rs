//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta421 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1369;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta421<F: Float>(t3362: F, t414: F, t66: F, t42859: F, t460: F, t42865: F, t479: F, t1244: F, t42871: F, t471: F, t12884: F, t828: F) -> (F, F, F, F, F, F) {
        let (t44362, t44372, t44373, t44375, t44378, t44425) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1369::<F>(t3362, t414, t66, t42859, t460, t42865, t479, t1244, t42871, t471, t12884, t828);
    (t44362, t44372, t44373, t44375, t44378, t44425)
}
