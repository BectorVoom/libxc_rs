//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta426 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1376;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta426<F: Float>(t3475: F, t426: F, t3478: F, t43752: F, t439: F, t3519: F, t3522: F, t43813: F, t1209: F, t13126: F, t17708: F, t44842: F, t487: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t45157, t45159, t45177, t45187, t45188, t45190, t45232, t45371, t45438) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1376::<F>(t3475, t426, t3478, t43752, t439, t3519, t3522, t43813, t1209, t13126, t17708, t44842, t487);
    (t45157, t45159, t45177, t45187, t45188, t45190, t45232, t45371, t45438)
}
