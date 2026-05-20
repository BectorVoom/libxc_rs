//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta111 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk724;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk725;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk726;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk727;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk728;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta111<F: Float>(t2297: F, t910: F, t914: F, t287: F, t913: F, t275: F, t273: F, t276: F, t2846: F) -> (F, F, F, F, F, F, F, F, F) {
        let t2857 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk724::<F>(t2297);
        let (t2869, t2872, t2873) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk725::<F>(t910, t914, t287, t913);
        let t2874 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk726::<F>(t275, t2873);
        let t2880 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk727::<F>(t273, t276);
        let (t2884, t2892, t2897) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk728::<F>(t2846, t273);
    (t2857, t2869, t2872, t2873, t2874, t2880, t2884, t2892, t2897)
}
