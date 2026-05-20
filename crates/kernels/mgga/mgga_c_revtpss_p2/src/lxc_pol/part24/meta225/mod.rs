//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta225 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk979;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk980;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta225<F: Float>(t12552: F, t439: F, t3522: F, t447: F, t3800: F, t498: F, t12295: F, t1207: F, t456: F) -> (F, F, F, F, F, F, F) {
        let (t12553, t12555) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk979::<F>(t12552, t439, t3522, t447);
        let (t12587, t12610, t12625, t12626, t12627) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk980::<F>(t3800, t498, t12295, t1207, t456);
    (t12553, t12555, t12587, t12610, t12625, t12626, t12627)
}
