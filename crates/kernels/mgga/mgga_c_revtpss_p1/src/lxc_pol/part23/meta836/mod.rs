//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta836 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2707;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta836<F: Float>(t3520: F, t6513: F, t3495: F, t3476: F, t6481: F, t20520: F, t3479: F, t3451: F, t20382: F, t3523: F, t12555: F, t6534: F) -> (F, F, F, F, F, F, F) {
        let (t69359, t69371, t69376, t69411, t69488, t69504, t69511) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2707::<F>(t3520, t6513, t3495, t3476, t6481, t20520, t3479, t3451, t20382, t3523, t12555, t6534);
    (t69359, t69371, t69376, t69411, t69488, t69504, t69511)
}
