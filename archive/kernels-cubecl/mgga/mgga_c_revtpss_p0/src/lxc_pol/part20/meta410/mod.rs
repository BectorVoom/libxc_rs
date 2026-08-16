//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta410 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1520;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta410<F: Float>(t342: F, t42859: F, t11626: F, t358: F, t3145: F, t365: F, t360: F, t3151: F, t373: F, t3153: F) -> (F, F, F, F, F, F, F, F) {
        let (t42860, t42862, t42865, t42866, t42868, t42869, t42870, t42871) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1520::<F>(t342, t42859, t11626, t358, t3145, t365, t360, t3151, t373, t3153);
    (t42860, t42862, t42865, t42866, t42868, t42869, t42870, t42871)
}
