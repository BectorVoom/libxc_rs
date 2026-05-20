//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta361 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1882;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1883;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1884;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta361<F: Float>(t1175: F, t3495: F, t3515: F, t3523: F, t1156: F, t3451: F, t12295: F, t12351: F, t1178: F, t3519: F, t439: F, t3522: F, t447: F) -> (F, F, F, F, F, F, F, F) {
        let (t12491, t12500, t12511, t12542, t12543, t12552) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1882::<F>(t1175, t3495, t3515, t3523, t1156, t3451, t12295, t12351, t1178, t3519);
        let t12553 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1883::<F>(t12552, t439);
        let t12555 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1884::<F>(t3522, t447);
    (t12491, t12500, t12511, t12542, t12543, t12552, t12553, t12555)
}
