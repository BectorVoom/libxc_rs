//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta83 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk496;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk497;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk498;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk499;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk500;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta83<F: Float>(t676: F, t886: F, t123: F, t2465: F, t215: F, t685: F, t788: F, t787: F, t206: F, t242: F, t240: F, t72: F, t2394: F, t828: F, t225: F, t786: F, t27: F, t823: F, t136: F, t826: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t2467, t2468, t2470) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk496::<F>(t676, t886, t123, t2465, t215, t685);
        let (t2471, t2473, t2475) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk497::<F>(t2470, t788, t787, t206, t242);
        let (t2476, t2477, t2479, t2482) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk498::<F>(t240, t2475, t72, t2394, t828, t225, t786);
        let t2484 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk499::<F>(t2482, t27, t823);
        let t2485 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk500::<F>(t136, t826);
    (t2467, t2468, t2470, t2471, t2473, t2475, t2476, t2477, t2479, t2482, t2484, t2485)
}
