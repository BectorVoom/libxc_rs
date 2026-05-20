//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta400 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1766;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1767;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1768;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1769;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta400<F: Float>(t17727: F, t17728: F, t3566: F, t489: F, t1121: F, t1774: F, t13142: F, t17708: F, t13127: F) -> (F, F, F, F, F, F) {
        let t17729 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1766::<F>(t17727, t17728);
        let (t17735, t17736) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1767::<F>(t3566, t489, t17728);
        let (t17737, t17747) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1768::<F>(t1121, t1774, t13142, t17708);
        let t17753 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1769::<F>(t13127, t17708);
    (t17729, t17735, t17736, t17737, t17747, t17753)
}
