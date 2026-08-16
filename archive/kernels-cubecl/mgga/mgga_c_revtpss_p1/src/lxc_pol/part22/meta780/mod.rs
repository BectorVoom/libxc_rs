//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta780 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2870;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta780<F: Float>(t45186: F, t439: F, t3522: F, t3444: F, t3451: F, t1156: F, t12428: F, t43813: F, t1209: F, t13126: F, t17708: F, t1203: F, t12626: F) -> (F, F, F, F, F, F, F, F) {
        let (t45187, t45188, t45190, t45194, t45197, t45232, t45371, t45384) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2870::<F>(t45186, t439, t3522, t3444, t3451, t1156, t12428, t43813, t1209, t13126, t17708, t1203, t12626);
    (t45187, t45188, t45190, t45194, t45197, t45232, t45371, t45384)
}
