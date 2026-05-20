//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta104 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk689;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk690;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk691;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk692;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk693;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta104<F: Float>(t243: F, t2668: F, t816: F, t813: F, t2482: F, t27: F, t849: F, t136: F, t854: F, t221: F, t775: F, t26: F, t66: F, t240: F, t247: F, t237: F, t124: F, t212: F, t596: F, t800: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t2672, t2674) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk689::<F>(t243, t2668, t816, t813, t2482, t27, t849);
        let t2675 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk690::<F>(t136, t854);
        let (t2677, t2678, t2681) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk691::<F>(t221, t2675, t775, t2674, t26, t66);
        let t2682 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk692::<F>(t240, t2681);
        let (t2686, t2689) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk693::<F>(t243, t247, t2682, t237, t124, t212, t596, t800);
    (t2672, t2674, t2675, t2677, t2678, t2681, t2682, t2686, t2689)
}
