//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta682 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2423;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta682<F: Float>(t43813: F, t43816: F, t3475: F, t426: F, t3478: F, t1175: F, t12552: F, t43752: F, t439: F, t3519: F, t3522: F, t1156: F, t12428: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t45106, t45107, t45157, t45159, t45174, t45177, t45187, t45188, t45190, t45197) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2423::<F>(t43813, t43816, t3475, t426, t3478, t1175, t12552, t43752, t439, t3519, t3522, t1156, t12428);
    (t45106, t45107, t45157, t45159, t45174, t45177, t45187, t45188, t45190, t45197)
}
