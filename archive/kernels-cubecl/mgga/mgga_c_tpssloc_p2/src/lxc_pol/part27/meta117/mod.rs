//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta117 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk700;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk701;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk702;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk703;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk704;
use chunk5::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk705;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta117<F: Float>(t2617: F, t816: F, t809: F, t838: F, t842: F, t233: F, t813: F, t236: F, t240: F, t812: F, t828: F, t232: F, t819: F, t820: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t2618, t2621, t2623, t2627) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk700::<F>(t2617, t816, t809, t838, t842, t233, t813);
        let t2628 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk701::<F>(t236, t2627);
        let (t2629, t2630, t2631) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk702::<F>(t240, t2628, t812, t828);
        let t2632 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk703::<F>(t232);
        let t2633 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk704::<F>(t2631, t2632);
        let t2635 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk705::<F>(t2633, t819, t820);
    (t2618, t2621, t2623, t2627, t2628, t2629, t2630, t2631, t2632, t2633, t2635)
}
