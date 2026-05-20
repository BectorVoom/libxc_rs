//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta362 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1885;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1886;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1887;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1888;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta362<F: Float>(t300: F, t3488: F, t3800: F, t498: F, t1204: F, t1269: F, t12295: F, t1207: F, t456: F, t487: F, t3566: F, t1203: F, t3565: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t12571, t12587, t12603, t12610, t12625, t12626, t12627) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1885::<F>(t300, t3488, t3800, t498, t1204, t1269, t12295, t1207, t456);
        let (t12628, t12633) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1886::<F>(t12627, t487, t1269, t3566);
        let t12640 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1887::<F>(t1203, t3565);
        let t12641 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1888::<F>(t12640, t487);
    (t12571, t12587, t12603, t12610, t12625, t12626, t12627, t12628, t12633, t12640, t12641)
}
