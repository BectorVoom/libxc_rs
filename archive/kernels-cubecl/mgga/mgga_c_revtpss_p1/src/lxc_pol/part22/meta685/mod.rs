//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta685 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2673;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta685<F: Float>(t21829: F, t665: F, t10227: F, t5895: F, t658: F, t1504: F, t2: F, t580: F, t2349: F, t5823: F, t9342: F, t100: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t21830, t21835, t21836, t21839, t21840, t21845, t21846, t21850, t21851) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2673::<F>(t21829, t665, t10227, t5895, t658, t1504, t2, t580, t2349, t5823, t9342, t100);
    (t21830, t21835, t21836, t21839, t21840, t21845, t21846, t21850, t21851)
}
