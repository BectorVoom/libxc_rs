//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta107 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk658;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk659;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk660;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk661;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk662;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta107<F: Float>(t2465: F, t2467: F, t215: F, t685: F, t788: F, t787: F, t206: F, t242: F, t240: F, t72: F, t2394: F, t828: F, t225: F, t786: F, t27: F, t823: F, t136: F, t826: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t2468, t2470) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk658::<F>(t2465, t2467, t215, t685);
        let t2471 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk659::<F>(t2470, t788);
        let (t2473, t2475, t2476, t2477, t2479, t2482) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk660::<F>(t2471, t787, t206, t242, t240, t72, t2394, t828, t225, t786);
        let t2484 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk661::<F>(t2482, t27, t823);
        let t2485 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk662::<F>(t136, t826);
    (t2468, t2470, t2471, t2473, t2475, t2476, t2477, t2479, t2482, t2484, t2485)
}
