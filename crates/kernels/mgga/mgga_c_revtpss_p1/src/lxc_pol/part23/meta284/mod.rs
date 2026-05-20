//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta284 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1509;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta284<F: Float>(t22: F, t780: F, t10981: F, t2455: F, t9285: F, t2454: F, t252: F, t2769: F, t786: F) -> (F, F, F, F, F, F) {
        let (t10982, t10984, t10985, t10987, t10994, t10995) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1509::<F>(t22, t780, t10981, t2455, t9285, t2454, t252, t2769, t786);
    (t10982, t10984, t10985, t10987, t10994, t10995)
}
